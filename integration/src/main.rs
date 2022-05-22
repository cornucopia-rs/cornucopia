use std::{borrow::Cow, process::ExitCode};

use clap::Parser;
use owo_colors::OwoColorize;

/// Start cornucopia test runner
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Format test descriptors and update error msg
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

fn main() -> ExitCode {
    let args = Args::parse();
    if run_test(args.apply) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

// Run test, return true if all test are successful
fn run_test(apply: bool) -> bool {
    let mut successful = true;

    let got_msg = if apply {
        "Apply:".bright_black()
    } else {
        "Got:".bright_black()
    };
    let expected_msg = if apply {
        "Previous:".bright_black()
    } else {
        "Expected:".bright_black()
    };

    // TODO reuse same connection using cornucopia lib API

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

            // We need to change current dir for error path to always be the same
            std::env::set_current_dir(&temp_dir).unwrap();

            // Generate migrations files
            std::fs::create_dir("migrations").unwrap();
            if let Some(migration) = test.migration {
                let name = test.migration_name.unwrap_or("1653210840_first.sql");
                std::fs::write(&format!("migrations/{name}"), migration).unwrap();
            }

            // generate queries files
            std::fs::create_dir("queries").unwrap();
            if let Some(query) = test.query {
                let name = test.query_name.unwrap_or("module_1.sql");
                std::fs::write(&format!("queries/{name}"), query).unwrap();
            }

            // Run command
            let output = std::process::Command::new("cornucopia")
                .arg("generate")
                .output()
                .unwrap();

            let err = String::from_utf8(output.stderr).unwrap();
            if err.trim() != test.error.trim() {
                successful = false;
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
            if apply {
                test.error = Cow::Owned(err.trim().to_string())
            }
            std::env::set_current_dir(&original_pwd).unwrap();
        }

        if apply {
            // Format test descriptor and update error message if needed
            let edited = toml::to_string_pretty(&suite).unwrap();
            std::fs::write(file.path(), edited).unwrap()
        }
    }
    return successful;
}

#[cfg(test)]
mod test {
    use crate::run_test;

    #[test]
    fn run() {
        assert!(run_test(false))
    }
}
