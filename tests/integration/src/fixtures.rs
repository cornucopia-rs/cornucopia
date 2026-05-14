use std::{
    error::Error,
    path::{Path, PathBuf},
};

use cornucopia::config::Config;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Serialize, Deserialize)]
struct TestSuiteSerde<T> {
    #[serde(rename = "test")]
    tests: Vec<T>,
}

pub struct TestSuite<T> {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) tests: Vec<T>,
}

impl<T: DeserializeOwned> TestSuite<T> {
    pub(crate) fn read<P: AsRef<Path>>(fixtures_path: P) -> impl Iterator<Item = TestSuite<T>> {
        std::fs::read_dir(fixtures_path).unwrap().map(|file| {
            let file = file.unwrap();
            let name = file.file_name().to_string_lossy().to_string();
            let path = file.path();
            let content = std::fs::read_to_string(&path).unwrap();
            let test_suite: TestSuiteSerde<T> = toml::from_str(&content).unwrap();
            TestSuite {
                name,
                tests: test_suite.tests,
                path,
            }
        })
    }
}

impl<T: Serialize> TestSuite<T> {
    pub(crate) fn write(self) -> Result<(), Box<dyn Error>> {
        let suite = TestSuiteSerde { tests: self.tests };
        let edited = toml::to_string_pretty(&suite)?;
        std::fs::write(self.path, edited)?;
        Ok(())
    }
}

/// Codegen test case
#[derive(Debug, Deserialize)]
pub(crate) struct CodegenTest {
    pub(crate) name: String,
    pub(crate) base_path: String,
    #[serde(default = "default_queries_path")]
    pub(crate) queries_path: PathBuf,
    pub(crate) destination: PathBuf,
    #[serde(default)]
    pub(crate) sync: bool,
    #[serde(default)]
    pub(crate) r#async: bool,
    #[serde(default)]
    pub(crate) run: bool,
    #[serde(default)]
    pub(crate) config: bool,
}

fn default_queries_path() -> PathBuf {
    PathBuf::from("queries/")
}

impl From<&CodegenTest> for Config {
    fn from(codegen_test: &CodegenTest) -> Self {
        match codegen_test.config {
            true => Config::from_file(Path::new("./cornucopia.toml")).unwrap(),
            false => Config::builder()
                .name(codegen_test.destination.to_str().unwrap())
                .destination(codegen_test.destination.clone())
                .queries(codegen_test.queries_path.clone())
                .r#async(codegen_test.r#async)
                .sync(codegen_test.sync)
                .build(),
        }
    }
}

/// Error test case
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ErrorTest {
    pub(crate) name: String,
    pub(crate) query: Option<String>,
    pub(crate) schema: Option<String>,
    pub(crate) error: String,
}

impl From<&ErrorTest> for Config {
    fn from(_error_test: &ErrorTest) -> Self {
        Config::builder()
            .r#async(false)
            .sync(true)
            .derive_traits(vec!["serde::Serialize".to_string()])
            .build()
    }
}
