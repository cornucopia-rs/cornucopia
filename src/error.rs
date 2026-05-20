use std::{io::IsTerminal, path::PathBuf};

use miette::{Diagnostic, GraphicalReportHandler, GraphicalTheme};
use thiserror::Error as ThisError;

/// Enumeration of all the warnings reported by Cornucopia.
#[derive(Debug, ThisError, Diagnostic)]
pub(crate) enum Warning {
    /// No annotated queries were found in the queries directory.
    #[error("no queries were found")]
    #[diagnostic(
        severity(Warning),
        help(
            "Cornucopia only generates code from annotated SQL queries. Make sure your queries directory path is correct and contains annotated queries. See https://cornucopia-rs.github.io/cornucopia/writing_queries/writing_queries.html for more."
        )
    )]
    NoQueries,
    /// User set `manifest.package.edition`, which cornucopia controls.
    #[error("`manifest.package.edition` is ignored")]
    #[diagnostic(
        severity(Warning),
        help(
            "Cornucopia controls the edition of the generated crate because the emitted code's syntax is tied to it. Remove this key from your config."
        )
    )]
    IgnoredManifestEdition,
    /// User set `manifest.package.rust-version`, which cornucopia controls.
    #[error("`manifest.package.rust-version` is ignored")]
    #[diagnostic(
        severity(Warning),
        help(
            "Cornucopia controls the MSRV of the generated crate because it is tied to the edition cornucopia emits for. Remove this key from your config."
        )
    )]
    IgnoredManifestRustVersion,
}

impl Warning {
    fn render(&self, theme: GraphicalTheme) -> String {
        let mut buff = String::new();
        if GraphicalReportHandler::new()
            .with_theme(theme)
            .render_report(&mut buff, self)
            .is_err()
        {
            format!("Warning: {self}")
        } else {
            buff
        }
    }

    /// Render this warning and write it to stderr, using ANSI colors when
    /// stderr is a terminal and `NO_COLOR` is not set.
    pub(crate) fn emit(&self) {
        let theme = if std::io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none() {
            GraphicalTheme::unicode()
        } else {
            GraphicalTheme::unicode_nocolor()
        };
        eprintln!("{}", self.render(theme));
    }
}

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
    ParseQueries(#[from] Box<crate::parser::error::Error>),
    /// An error while trying to validate PostgreSQL query files.
    ValidateQueries(#[from] Box<crate::validation::error::Error>),
    /// An error while manipulating a container managed by Cornucopia.
    Container(#[from] crate::container::error::Error),
    /// An error while trying to prepare PostgreSQL queries.
    PrepareQueries(#[from] Box<crate::prepare_queries::error::Error>),
    /// An error while reading PostgreSQL schema files.
    LoadSchema(#[from] Box<crate::load_schema::error::Error>),
    /// An error while trying to write the generated crate to its destination.
    PersistCrate(#[from] PersistError),
    /// An error while trying to read the config flle
    Config(#[from] crate::config::ConfigError),
}

impl Error {
    #[must_use]
    pub fn report(self) -> String {
        let mut buff = String::new();
        if GraphicalReportHandler::new()
            .with_theme(GraphicalTheme::unicode_nocolor())
            .render_report(&mut buff, &self)
            .is_err()
        {
            format!("Error: {self}")
        } else {
            buff
        }
    }
}

#[derive(Debug, ThisError, Diagnostic)]
#[error("Could not perform IO on file `{file_path}`: ({err})")]
pub struct PersistError {
    pub(crate) file_path: PathBuf,
    pub(crate) err: std::io::Error,
}

impl PersistError {
    pub fn wrap(path: impl Into<PathBuf>) -> impl FnOnce(std::io::Error) -> PersistError {
        |err| PersistError {
            file_path: path.into(),
            err,
        }
    }
}

#[cfg(test)]
mod tests {
    use miette::GraphicalTheme;

    use super::Warning;

    #[test]
    fn no_queries_warning_renders() {
        let rendered = Warning::NoQueries.render(GraphicalTheme::unicode_nocolor());
        assert!(rendered.contains("no queries were found"));
        assert!(rendered.contains("annotated SQL queries"));
    }
}
