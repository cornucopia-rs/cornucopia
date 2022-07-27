use miette::NamedSource;
use postgres::Client;

use crate::utils::db_err;

use self::error::Error;

#[derive(Debug, Clone)]
pub struct Schema {
    pub path: String,
    pub sql: String,
}

impl From<Schema> for NamedSource {
    fn from(m: Schema) -> Self {
        Self::new(m.path, m.sql)
    }
}

fn read_schema(path: String) -> Result<Schema, Error> {
    match std::fs::read_to_string(&path) {
        Ok(sql) => Ok(Schema { path, sql }),
        Err(err) => Err(Error::Io { path, err }),
    }
}

fn execute_schema(client: &mut Client, schema: Schema) -> Result<(), Error> {
    client.batch_execute(&schema.sql).map_err(|err| {
        let msg = format!("{:#}", err);
        if let Some((position, msg, help)) = db_err(&err) {
            Error::Postgres {
                msg,
                help,
                src: schema.into(),
                err_span: Some((position as usize..position as usize).into()),
            }
        } else {
            Error::Postgres {
                msg,
                help: None,
                src: schema.into(),
                err_span: None,
            }
        }
    })
}

/// Reads schema files from arg paths, if the path is a file we read it, if the path is a directory we read all .sql files it contains.
pub fn read_schemas(client: &mut Client, paths: Vec<String>) -> Result<(), Error> {
    for path in paths {
        let metadata = std::fs::metadata(&path).map_err(|err| Error::Io {
            path: path.clone(),
            err,
        })?;
        if metadata.is_file() {
            execute_schema(client, read_schema(path)?)?
        } else {
            let mut files = std::fs::read_dir(&path)
                .map_err(|err| Error::Io {
                    path: path.clone(),
                    err,
                })
                .and_then(|dir| {
                    dir.filter_map(|it| {
                        it.map_err(|err| Error::Io {
                            path: path.clone(),
                            err,
                        })
                        .and_then(|entry| {
                            let path = entry.path();
                            (path.extension().and_then(|it| it.to_str()) == Some("sql"))
                                .then(|| read_schema(path.to_string_lossy().to_string()))
                                .transpose()
                        })
                        .transpose()
                    })
                    .collect::<Result<Vec<Schema>, Error>>()
                })?;
            files.sort_by(|a, b| a.path.cmp(&b.path));
            for schema in files {
                execute_schema(client, schema)?;
            }
        }
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
