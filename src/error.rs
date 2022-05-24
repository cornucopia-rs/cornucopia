use crate::codegen::error::Error as CodegenError;
use crate::container::error::Error as ContainerError;
use crate::prepare_queries::error::Error as PrepareQueriesError;
use crate::read_queries::error::Error as ReadQueriesError;
use crate::run_migrations::error::Error as MigrationError;

use thiserror::Error as ThisError;
#[derive(Debug, ThisError)]
#[error("{0}")]
pub enum Error {
    ReadQueries(#[from] ReadQueriesError),
    Container(#[from] ContainerError),
    Codegen(#[from] CodegenError),
    PrepareQueries(#[from] PrepareQueriesError),
    NewMigration(#[from] NewMigrationError),
    Migration(#[from] MigrationError),
    Db(#[from] tokio_postgres::Error),
    WriteCodeGenFile(#[from] WriteCodeGenFileError),
}

#[derive(Debug, ThisError)]
#[error("Could not write your queries to destination file `{file_path}`: ({err})")]
pub struct WriteCodeGenFileError {
    pub(crate) file_path: String,
    pub(crate) err: std::io::Error,
}

#[derive(Debug, ThisError)]
#[error("Could not create new migration `{file_path}`: ({err})")]
pub struct NewMigrationError {
    pub(crate) file_path: String,
    pub(crate) err: std::io::Error,
}
