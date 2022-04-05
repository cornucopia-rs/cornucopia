use crate::parse::ParsedQuery;
use crate::parse::Quantifier;
use crate::parse::ReturnType;
use crate::parse::TypedParam;
use deadpool_postgres::{Client, Object};
use error::Error;
use tokio_postgres::types::Type;

use super::read_queries::{read_queries, Module};

#[derive(Debug)]
pub struct PreparedQuery {
    pub name: String,
    pub override_types: Vec<Type>,
    pub params: Vec<TypedParam>,
    pub ret: RustReturnType,
    pub quantifier: Quantifier,
    pub sql: String,
}

#[derive(Debug)]
pub struct PreparedModule {
    pub name: String,
    pub queries: Vec<PreparedQuery>,
}

pub async fn prepare_modules(client: &Object, path: &str) -> Result<Vec<PreparedModule>, Error> {
    let mut prepared_modules = Vec::new();
    for module in read_queries(path)? {
        prepared_modules.push(prepare_module(client, module).await?);
    }
    Ok(prepared_modules)
}

async fn prepare_module(client: &Object, module: Module) -> Result<PreparedModule, Error> {
    let mut queries = Vec::new();
    for query in module.queries {
        queries.push(prepare_query(client, query).await?);
    }
    Ok(PreparedModule {
        name: module.name,
        queries,
    })
}

async fn prepare_query(client: &Client, query: ParsedQuery) -> Result<PreparedQuery, Error> {
    let stmt = client
        .prepare_typed(&query.sql, &query.meta.override_types)
        .await?;
    let params = query
        .meta
        .params
        .into_iter()
        .zip(stmt.params().iter().cloned())
        .map(|(name, ty)| TypedParam { name, ty })
        .collect::<Vec<TypedParam>>();

    let ret = {
        let mut return_types = stmt
            .columns()
            .iter()
            .map(|c| c.type_().clone())
            .collect::<Vec<Type>>();
        match query.meta.ret {
            ReturnType::Implicit => match return_types.len() {
                0 => RustReturnType::Void,
                // ![unwrap] This is ok because we just checked that we do have one element.
                1 => RustReturnType::Scalar(return_types.pop().unwrap()),
                _ => RustReturnType::Tuple(return_types),
            },
            ReturnType::Explicit { field_names } => {
                let fields = field_names
                    .into_iter()
                    .zip(return_types)
                    .map(|(name, ty)| TypedParam { name, ty })
                    .collect::<Vec<TypedParam>>();
                RustReturnType::Struct(fields)
            }
        }
    };

    Ok(PreparedQuery {
        name: query.meta.name,
        override_types: query.meta.override_types,
        params,
        ret,
        quantifier: query.meta.quantifier,
        sql: query.sql,
    })
}
#[derive(Debug)]
pub enum RustReturnType {
    Void,
    Scalar(Type),
    Tuple(Vec<Type>),
    Struct(Vec<TypedParam>),
}

pub mod error {
    use crate::read_queries::error::Error as ReadQueriesError;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    pub enum Error {
        #[error("")]
        Db(#[from] tokio_postgres::Error),
        #[error("")]
        Pool(#[from] deadpool_postgres::PoolError),
        #[error("")]
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
        #[error("Cannot parse '{ty}' into postgres type")]
        UnrecognizedType { ty: String },
    }
}
