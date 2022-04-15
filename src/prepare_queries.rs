use crate::parse::ParsedQuery;
use crate::parse::Quantifier;
use crate::parse::ReturnType;
use crate::pg_type::CornucopiaField;
use crate::pg_type::CornucopiaType;
use crate::pg_type::TypeRegistrar;
use deadpool_postgres::{Client, Object};
use error::Error;
use postgres_types::Kind;
use tokio_postgres::types::Type;

use super::read_queries::{read_queries, Module};

#[derive(Debug)]
pub struct PreparedQuery {
    pub name: String,
    pub override_types: Vec<CornucopiaType>,
    pub params: Vec<CornucopiaField>,
    pub ret: RustReturnType,
    pub quantifier: Quantifier,
    pub sql: String,
}

#[derive(Debug)]
pub struct PreparedModule {
    pub name: String,
    pub queries: Vec<PreparedQuery>,
}

#[derive(Debug)]
pub enum RustReturnType {
    Void,
    Scalar(CornucopiaType),
    Tuple(Vec<CornucopiaType>),
    Struct(Vec<CornucopiaField>),
}

pub async fn prepare_modules(
    type_registrar: &mut TypeRegistrar,
    client: &Object,
    path: &str,
) -> Result<Vec<PreparedModule>, Error> {
    let mut prepared_modules = Vec::new();
    for module in read_queries(type_registrar, path)? {
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
        queries.push(prepare_query(client, query, type_registrar).await?);
    }
    Ok(PreparedModule {
        name: module.name,
        queries,
    })
}

async fn prepare_query(
    client: &Client,
    query: ParsedQuery,
    type_registrar: &mut TypeRegistrar,
) -> Result<PreparedQuery, Error> {
    let mut override_types = Vec::new();
    for override_type in query.meta.override_types {
        let override_type = type_registrar
            .register(client, override_type.schema, override_type.name)
            .await?;
        override_types.push(override_type);
    }
    let override_pg_types = override_types
        .iter()
        .map(|t| t.pg_ty.clone())
        .collect::<Vec<Type>>();
    let stmt = client
        .prepare_typed(&query.sql, override_pg_types.as_slice())
        .await?;

    let mut param_types = Vec::new();
    for param in stmt.params() {
        let param_type = type_registrar.register_type(client, param).await?;
        param_types.push(param_type);
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
            let ty = type_registrar.register_type(client, column.type_()).await?;
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
            ReturnType::Explicit { field_names } => {
                let fields = field_names
                    .into_iter()
                    .zip(return_types)
                    .map(|(name, ty)| CornucopiaField { name, ty })
                    .collect::<Vec<CornucopiaField>>();
                RustReturnType::Struct(fields)
            }
        }
    };

    Ok(PreparedQuery {
        name: query.meta.name,
        override_types,
        params,
        ret,
        quantifier: query.meta.quantifier,
        sql: query.sql,
    })
}

pub mod error {
    use crate::pg_type::error::Error as PostgresTypeError;
    use crate::read_queries::error::Error as ReadQueriesError;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("encountered error while preparing queries")]
    pub enum Error {
        Db(#[from] tokio_postgres::Error),
        Pool(#[from] deadpool_postgres::PoolError),
        ReadQueries(#[from] ReadQueriesError),
        #[error(
            "invalid number of parameters in {module}::{name}. Expected {expected}, got {actual}"
        )]
        NbParameters {
            module: String,
            name: String,
            expected: usize,
            actual: usize,
        },
        PostgresType(#[from] PostgresTypeError),
    }
}

pub struct Allo {
    pub bob: std::sync::Arc<Vec<String>>,
}
