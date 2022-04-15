use crate::parse::ParsedQuery;
use crate::parse_file::parse_file;
use error::Error;

#[derive(Debug)]
pub(crate) struct Module {
    pub(crate) name: String,
    pub(crate) queries: Vec<ParsedQuery>,
}

pub(crate) fn read_queries(path: &str) -> Result<Vec<Module>, Error> {
    let mut modules = Vec::new();
    for entry_result in std::fs::read_dir(path)? {
        let entry = entry_result?;
        let path = entry.path();

        if path
            .extension()
            .map(|extension| extension == "sql")
            .unwrap_or_default()
        {
            let module_name = path
                .file_stem()
                .expect("is a file")
                .to_str()
                .expect("file stem is valid utf8")
                .to_string();

            let module = Module {
                name: module_name,
                queries: parse_file(&path)?,
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
    #[error("error while reading migrations")]
    pub enum Error {
        Io(#[from] std::io::Error),
        Parser(#[from] FileParserError),
    }
}
