use crate::{
    fixtures::{CodegenTest, TestSuite},
    utils::reset_db,
};

use cornucopia::{Error, config::Config};
use owo_colors::OwoColorize;
use std::{env::set_current_dir, path::PathBuf, process::Command};
use tempfile::tempdir;

// Run codegen test, return true if all test are successful
pub(crate) fn run_codegen_test(
    client: &tokio_postgres::Client,
    apply: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut successful = true;
    let original_pwd = std::env::current_dir()?;
    let fixtures_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/codegen");
    let test_suites = TestSuite::<CodegenTest>::read(fixtures_path);

    let tmp_dir = tempdir()?;
    for suite in test_suites {
        println!("{}", format!("[codegen] {}", suite.name).magenta());
        for test in suite.tests {
            // Reset DB
            reset_db(client)?;

            // Set current dir to test base path
            set_current_dir(
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../{}", test.base_path)),
            )?;

            // Load schema
            cornucopia::load_schema(client, &["schema.sql"])?;

            // If `--apply`, then the code will be regenerated.
            // Otherwise, it is only checked.
            if apply {
                // Generate
                cornucopia::gen_live(client, Config::from(&test)).map_err(Error::report)?;
            } else {
                let tmp_path = tmp_dir.path().join(
                    test.destination
                        .file_name()
                        .unwrap_or("cornucopia".as_ref()),
                );

                std::fs::create_dir(&tmp_path)?;

                let mut cfg = Config::from(&test);
                cfg.destination = tmp_path.clone();

                cornucopia::gen_live(client, cfg).map_err(Error::report)?;
            }

            println!("(generate) {} {}", test.name, "OK".green());

            if test.run {
                // Change current directory
                set_current_dir(
                    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                        .join(format!("../../{}", test.base_path)),
                )?;

                // Run
                let result = Command::new("cargo").arg("run").output()?;
                if result.status.success() {
                    println!("(run) {} {}", test.name, "OK".green());
                } else {
                    successful = false;
                    println!(
                        " {}\n{}",
                        "ERR".red(),
                        String::from_utf8_lossy(&result.stderr)
                            .as_ref()
                            .bright_black()
                    );
                }
            }

            // Move back to original directory
            std::env::set_current_dir(&original_pwd)?;
        }
    }

    Ok(successful)
}
