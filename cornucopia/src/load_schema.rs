use miette::NamedSource;
use postgres::Client;

use crate::utils::db_err;

use self::error::Error;

/// Load schema into a database. Take a list of files paths as arguments and load them in their given order.
pub fn load_schema(client: &mut Client, paths: Vec<String>) -> Result<(), Error> {
    for path in paths {
        let sql = std::fs::read_to_string(&path).map_err(|err| Error::Io {
            path: path.clone(),
            err,
        })?;
        client.batch_execute(&sql).map_err(|err| {
            let msg = format!("{:#}", err);
            let src = NamedSource::new(path, sql);
            if let Some((position, msg, help)) = db_err(&err) {
                Error::Postgres {
                    msg,
                    help,
                    src,
                    err_span: Some((position as usize..position as usize).into()),
                }
            } else {
                Error::Postgres {
                    msg,
                    help: None,
                    src,
                    err_span: None,
                }
            }
        })?;
    }
    Ok(())
}

pub(crate) mod error {
    use miette::{Diagnostic, NamedSource, SourceSpan};
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError, Diagnostic)]
    pub enum Error {
        #[error("Could not read schema `{path}`: ({err})")]
        Io { path: String, err: std::io::Error },
        #[error("Could not execute schema: {msg}")]
        Postgres {
            msg: String,
            #[source_code]
            src: NamedSource,
            #[help]
            help: Option<String>,
            #[label("error occurs near this location")]
            err_span: Option<SourceSpan>,
        },
    }
}
