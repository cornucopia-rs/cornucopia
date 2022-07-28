use miette::{Diagnostic, GraphicalReportHandler, GraphicalTheme};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum Error {
    Connection(#[from] crate::conn::error::Error),
    ReadQueries(#[from] crate::read_queries::error::Error),
    ParseQueries(#[from] crate::parser::error::Error),
    ValidateQueries(#[from] crate::validation::error::Error),
    Container(#[from] crate::container::error::Error),
    PrepareQueries(#[from] crate::prepare_queries::error::Error),
    LoadSchema(#[from] crate::load_schema::error::Error),
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
