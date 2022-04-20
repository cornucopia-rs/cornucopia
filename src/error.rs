use crate::codegen::error::Error as CodegenError;
use crate::container::error::Error as ContainerError;
use crate::pool::error::Error as PoolBuilderError;
use crate::prepare_queries::error::Error as PrepareQueriesError;
use crate::read_queries::error::Error as ReadQueriesError;
use crate::run_migrations::error::Error as MigrationError;

use deadpool_postgres::CreatePoolError;
use thiserror::Error as ThisError;
#[derive(Debug, ThisError)]
#[error("the program encountered an unexpected error")]
pub(crate) enum Error {
    ReadQueries(#[from] ReadQueriesError),
    Container(#[from] ContainerError),
    Codegen(#[from] CodegenError),
    PrepareQueries(#[from] PrepareQueriesError),
    NewMigration(#[from] std::io::Error),
    Migration(#[from] MigrationError),
    PoolCreation(#[from] CreatePoolError),
    Pool(#[from] deadpool_postgres::PoolError),
    PoolBuilder(#[from] PoolBuilderError),
    FmtError(#[from] FmtError),
}

#[derive(Debug, ThisError)]
#[error("`rustfmt` was unable to properly format the generated code. This is probably a bug, and you should report it")]
pub(crate) enum FmtError {
    IO(#[from] std::io::Error),
    #[error("bad code")]
    RustFmt,
}
