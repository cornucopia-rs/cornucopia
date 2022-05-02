use crate::parse::ExplicitReturnParam;
use crate::parse::ParsedQuery;
use crate::parse::Quantifier;
use crate::parse::ReturnType;
use crate::pg_type::CornucopiaField;
use crate::pg_type::CornucopiaType;
use crate::pg_type::TypeRegistrar;
use deadpool_postgres::{Client, Object};
use error::Error;

use error::ErrorVariant;

use super::read_queries::Module;

#[derive(Debug)]
pub(crate) struct PreparedQuery {
    pub(crate) name: String,
    pub(crate) params: Vec<CornucopiaField>,
    pub(crate) ret: RustReturnType,
    pub(crate) quantifier: Quantifier,
    pub(crate) sql: String,
}

#[derive(Debug)]
pub(crate) struct PreparedModule {
    pub(crate) name: String,
    pub(crate) queries: Vec<PreparedQuery>,
}

#[derive(Debug)]
pub(crate) enum RustReturnType {
    Void,
    Scalar(CornucopiaType),
    Tuple(Vec<CornucopiaType>),
    Struct(Vec<(ExplicitReturnParam, CornucopiaType)>),
}

pub(crate) async fn prepare_modules(
    client: &Object,
    type_registrar: &mut TypeRegistrar,
    modules: Vec<Module>,
) -> Result<Vec<PreparedModule>, Error> {
    let mut prepared_modules = Vec::new();
    for module in modules {
        prepared_modules.push(prepare_module(client, module, type_registrar).await?);
    }
    Ok(prepared_modules)
}

async fn prepare_module(
    client: &Object,
    module: Module,
    type_registrar: &mut TypeRegistrar,
) -> Result<PreparedModule, Error> {
    let mut queries = Vec::new();
    for query in module.queries {
        queries.push(prepare_query(client, type_registrar, query, &module.path).await?);
    }
    Ok(PreparedModule {
        name: module.name,
        queries,
    })
}

async fn prepare_query(
    client: &Client,
    type_registrar: &mut TypeRegistrar,
    query: ParsedQuery,
    module_path: &str,
) -> Result<PreparedQuery, Error> {
    let stmt = client.prepare(&query.sql).await.map_err(|e| Error {
        line: Some(query.line),
        err: e.into(),
        path: String::from(module_path),
        name: query.meta.name.clone(),
    })?;

    let mut param_types = Vec::new();
    for param in stmt.params() {
        let param_type = type_registrar
            .register_type(client, param)
            .await
            .map_err(|e| Error {
                line: Some(query.line),
                err: e.into(),
                path: String::from(module_path),
                name: query.meta.name.clone(),
            })?;
        param_types.push(param_type);
    }

    if query.meta.params.len() != param_types.len() {
        return Err(Error {
            err: ErrorVariant::NbParameters {
                expected: param_types.len(),
                actual: query.meta.params.len(),
            },
            name: query.meta.name,
            line: Some(query.line),
            path: String::from(module_path),
        });
    }

    let params = query
        .meta
        .params
        .into_iter()
        .zip(param_types)
        .map(|(name, ty)| CornucopiaField { name, ty })
        .collect::<Vec<CornucopiaField>>();

    let ret = {
        let mut return_types = Vec::new();
        for column in stmt.columns() {
            let ty = type_registrar
                .register_type(client, column.type_())
                .await
                .map_err(|e| Error {
                    line: Some(query.line),
                    err: e.into(),
                    path: String::from(module_path),
                    name: query.meta.name.clone(),
                })?;
            return_types.push(ty);
        }

        match query.meta.ret {
            ReturnType::Implicit => match return_types.len() {
                0 => RustReturnType::Void,
                1 => RustReturnType::Scalar(
                    return_types
                        .pop()
                        .expect("moving out the single return type found"),
                ),
                _ => RustReturnType::Tuple(return_types),
            },
            ReturnType::Explicit { params } => {
                let fields = params
                    .into_iter()
                    .zip(return_types)
                    .map(|(param, ty)| (param, ty))
                    .collect::<Vec<(ExplicitReturnParam, CornucopiaType)>>();
                RustReturnType::Struct(fields)
            }
        }
    };

    Ok(PreparedQuery {
        name: query.meta.name,
        params,
        ret,
        quantifier: query.meta.quantifier,
        sql: query.sql,
    })
}

pub(crate) mod error {
    use std::fmt::Display;

    use crate::pg_type::error::Error as PostgresTypeError;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("{0}")]
    pub(crate) enum ErrorVariant {
        Db(#[from] tokio_postgres::Error),
        Pool(#[from] deadpool_postgres::PoolError),
        #[error("Invalid number of parameters (expected {expected}, got {actual})")]
        NbParameters {
            expected: usize,
            actual: usize,
        },
        PostgresType(#[from] PostgresTypeError),
    }

    #[derive(Debug)]
    pub(crate) struct Error {
        pub(crate) name: String,
        pub(crate) line: Option<usize>,
        pub(crate) err: ErrorVariant,
        pub(crate) path: String,
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self.err {
                ErrorVariant::Db(e) => write!(
                    f,
                    "Error while preparing query \"{}\" [file: \"{}\", line: {}] ({}).",
                    self.name,
                    self.path,
                    self.line.unwrap_or_default(),
                    e.as_db_error().unwrap().message()
                ),
                _ => match self.line {
                    Some(line) => {
                        write!(
                            f,
                            "Error while preparing query \"{}\" [file: \"{}\", line: {}]: {}.",
                            self.name, self.path, line, self.err
                        )
                    }
                    None => {
                        write!(
                            f,
                            "Error while preparing query \"{}\" [file: \"{}\"]: {}.",
                            self.name, self.path, self.err
                        )
                    }
                },
            }
        }
    }

    impl std::error::Error for Error {}
}
