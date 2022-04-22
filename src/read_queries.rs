use crate::parse::ParsedQuery;
use crate::parse_file::parse_file;

use self::error::*;

#[derive(Debug)]
pub(crate) struct Module {
    pub(crate) name: String,
    pub(crate) queries: Vec<ParsedQuery>,
}

pub(crate) fn read_queries(path: &str) -> Result<Vec<Module>, Error> {
    let mut modules = Vec::new();
    for entry_result in std::fs::read_dir(path).map_err(|err| Error::new(err.into(), path))? {
        let entry = entry_result.map_err(|err| Error::new(err.into(), path))?;
        let path_buf = entry.path();

        if path_buf
            .extension()
            .map(|extension| extension == "sql")
            .unwrap_or_default()
        {
            let module_name = path_buf
                .file_stem()
                .expect("is a file")
                .to_str()
                .expect("file stem is valid utf8")
                .to_string();

            let module = Module {
                name: module_name,
                queries: parse_file(&path_buf)
                    .map_err(|err| Error::new(err.into(), path_buf.to_str().unwrap()))?,
            };

            modules.push(module);
        } else {
            continue;
        }
    }
    Ok(modules)
}

pub(crate) mod error {
    use crate::parse_file::error::Error as FileParserError;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("{0}")]
    pub(crate) enum ErrorVariants {
        Io(#[from] std::io::Error),
        Parser(#[from] FileParserError),
    }

    #[derive(Debug, ThisError)]
    #[error("Error while reading query \"{path}\": {err}.")]
    pub(crate) struct Error {
        pub(crate) err: ErrorVariants,
        pub(crate) path: String,
    }

    impl Error {
        pub(crate) fn new(err: ErrorVariants, path: &str) -> Self {
            Self {
                path: std::fs::canonicalize(path)
                    .map(|p| p.to_str().unwrap().to_string())
                    .unwrap_or_else(|_| String::from(path)),
                err,
            }
        }
    }
}
