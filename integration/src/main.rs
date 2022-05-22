#[derive(serde::Deserialize)]
pub struct TestSuite<'a> {
    #[serde(borrow)]
    test: Vec<Test<'a>>,
}

#[derive(serde::Deserialize)]
pub struct Test<'a> {
    name: &'a str,
    query: Option<&'a str>,
    migration: Option<&'a str>,
    error: &'a str,
}

fn main() {
    // TODO use cornucopia lib API
    let original_pwd = std::env::current_dir().unwrap();
    for file in std::fs::read_dir("fixtures").unwrap() {
        let file = file.unwrap();
        let name = file.file_name().to_string_lossy().to_string();
        let content = std::fs::read_to_string(file.path()).unwrap();
        let suite: TestSuite = toml::from_str(&content).unwrap();

        println!("TEST SUITE {name}");
        for test in suite.test {
            // Generate file tree path
            let temp_dir = tempfile::tempdir().unwrap();

            std::env::set_current_dir(&temp_dir).unwrap();
            let queries_path = "queries";
            let migrations_path = "migrations";
            let out_path = "cornucopia.rs";

            // Generate file tree content
            std::fs::create_dir(&queries_path).unwrap();
            std::fs::create_dir(&migrations_path).unwrap();
            
            if let Some(migration) = test.migration {
                std::fs::write("migrations/1653210840_first.sql", migration).unwrap();
            }
            if let Some(query) = test.query {
                std::fs::write("queries/module_1.sql", query).unwrap();
            }

            let output = std::process::Command::new("cornucopia")
                .arg("generate")
                .arg("-q")
                .arg(queries_path)
                .arg("-m")
                .arg(migrations_path)
                .arg("-d")
                .arg(out_path)
                .output()
                .unwrap();

            let err = String::from_utf8(output.stderr).unwrap();
            if err.trim() != test.error.trim() {
                println!("{} Err\nExpected:\n{}\nGot:\n{err}", test.name, test.error);
            } else {
                println!("{} OK", test.name);
            }
            std::env::set_current_dir(&original_pwd).unwrap();
        }
    }
}
