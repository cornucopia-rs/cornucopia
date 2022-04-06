
use crate::codegen::error::Error as CodegenError;
use crate::container::error::Error as ContainerError;
use crate::prepare_queries::error::Error as PrepareQueriesError;
use crate::run_migrations::error::Error as MigrationError;
use deadpool_postgres::CreatePoolError;
use thiserror::Error as ThisError;
#[derive(Debug, ThisError)]
#[error("the program encountered an unexpected error")]
pub enum Error {
    ContainerError(#[from] ContainerError),
    Codegen(#[from] CodegenError),
    PrepareQueries(#[from] PrepareQueriesError),
    NewMigration(#[from] std::io::Error),
    Migration(#[from] MigrationError),
    PoolCreation(#[from] CreatePoolError),
    Pool(#[from] deadpool_postgres::PoolError),
}
