use std::borrow::Cow;

use clap::Parser;
use owo_colors::OwoColorize;

/// Run cornucopia test runner
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Apply returned error to the test description if not matching
    #[clap(short, long)]
    apply: bool,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TestSuite<'a> {
    #[serde(borrow)]
    test: Vec<Test<'a>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Test<'a> {
    name: &'a str,
    query: Option<&'a str>,
    migration: Option<&'a str>,
    query_name: Option<&'a str>,
    migration_name: Option<&'a str>,
    error: Cow<'a, str>,
}

fn main() {
    let args = Args::parse();

    let got_msg = if args.apply {
        "Apply:".bright_black()
    } else {
        "Got:".bright_black()
    };
    let expected_msg = if args.apply {
        "Previous:".bright_black()
    } else {
        "Expected:".bright_black()
    };

    // TODO use cornucopia lib API
    let original_pwd = std::env::current_dir().unwrap();
    for file in std::fs::read_dir("fixtures").unwrap() {
        let file = file.unwrap();
        let name = file.file_name().to_string_lossy().to_string();
        let content = std::fs::read_to_string(file.path()).unwrap();
        let mut suite: TestSuite = toml::from_str(&content).unwrap();

        println!("{}", name.magenta());
        for test in &mut suite.test {
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
                let name = test.migration_name.unwrap_or("1653210840_first.sql");
                std::fs::write(&format!("migrations/{name}"), migration).unwrap();
            }
            if let Some(query) = test.query {
                let name = test.query_name.unwrap_or("module_1.sql");
                std::fs::write(&format!("queries/{name}"), query).unwrap();
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
                println!(
                    "{} {}\n{}\n{}\n{}\n{}",
                    test.name,
                    "ERR".red(),
                    got_msg,
                    err,
                    expected_msg,
                    test.error
                );
            } else {
                println!("{} {}", test.name, "OK".green());
            }
            if args.apply {
                test.error = Cow::Owned(err.trim().to_string())
            }
            std::env::set_current_dir(&original_pwd).unwrap();
        }
        if args.apply {
            let edited = toml::to_string_pretty(&suite).unwrap();
            std::fs::write(file.path(), edited).unwrap()
        }
    }
}
