use crate::parse::ParsedQuery;
use crate::parse_file::parse_file;
use error::Error;

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub queries: Vec<ParsedQuery>,
}

pub fn read_queries(path: &str) -> Result<Vec<Module>, Error> {
    let mut modules = Vec::new();
    for entry_result in std::fs::read_dir(path)? {
        let entry = entry_result?;
        let path = entry.path();

        if path
            .extension()
            .map(|extension| extension == "sql")
            .unwrap_or_default()
        {
            // ![unwrap] We just checked that this is a file with an extension
            let module_name = path.file_stem().unwrap().to_str().unwrap().to_string();

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

pub mod error {
    use crate::parse_file::error::Error as FileParserError;
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("error while reading migrations")]
    pub enum Error {
        Io(#[from] std::io::Error),
        Parser(#[from] FileParserError),
    }
}
