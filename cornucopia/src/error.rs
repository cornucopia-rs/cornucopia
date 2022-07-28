use miette::{Diagnostic, GraphicalReportHandler, GraphicalTheme};
use thiserror::Error as ThisError;

/// Enumeration of all the errors reported by Cornucopia.
#[derive(Debug, ThisError, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum Error {
    /// An error while trying to connect to a database.
    Connection(#[from] crate::conn::error::Error),
    /// An error while trying to read PostgreSQL query files.
    ReadQueries(#[from] crate::read_queries::error::Error),
    /// An error while trying to parse PostgreSQL query files.
    ParseQueries(#[from] crate::parser::error::Error),
    /// An error while trying to validate PostgreSQL query files.
    ValidateQueries(#[from] crate::validation::error::Error),
    /// An error while manipulating a container managed by Cornucopia.
    Container(#[from] crate::container::error::Error),
    /// An error while trying to prepare PostgreSQL queries.
    PrepareQueries(#[from] crate::prepare_queries::error::Error),
    /// An error while reading PostgreSQL schema files.
    LoadSchema(#[from] crate::load_schema::error::Error),
    /// An error while trying to write the generated code to its destination file.
    WriteCodeGenFile(#[from] WriteOutputError),
}

impl Error {
    #[must_use]
    pub fn report(self) -> String {
        let mut buff = String::new();
        GraphicalReportHandler::new()
            .with_theme(GraphicalTheme::unicode_nocolor())
            .render_report(&mut buff, &self)
            .unwrap();
        buff
    }
}

#[derive(Debug, ThisError, Diagnostic)]
#[error("Could not write your queries to destination file `{file_path}`: ({err})")]
pub struct WriteOutputError {
    pub(crate) file_path: String,
    pub(crate) err: std::io::Error,
}
