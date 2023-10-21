use crate::{
    fixtures::{CodegenTest, TestSuite},
    utils::{reset_db, rustfmt_lib},
};

use cornucopia::{CodegenSettings, Error};
use owo_colors::OwoColorize;
use std::{env::set_current_dir, process::Command};
use tempfile::tempdir;

// Run codegen test, return true if all test are successful
pub(crate) fn run_codegen_test(
    client: &mut postgres::Client,
    apply: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut successful = true;
    let original_pwd = std::env::current_dir()?;
    let fixture_path = "fixtures/codegen";

    let test_suites = TestSuite::<CodegenTest>::read(fixture_path);
    let tmp_dir = tempdir()?;
    for suite in test_suites {
        println!("{}", format!("[codegen] {}", suite.name).magenta());
        for test in suite.tests {
            // Reset DB
            reset_db(client)?;

            // Set current dir to test base path
            set_current_dir(format!("../{}", test.base_path))?;

            // Load schema
            cornucopia::load_schema(client, &["schema.sql"])?;

            // If `--apply`, then the code will be regenerated.
            // Otherwise, it is only checked.
            if apply {
                // Generate
                cornucopia::gen_live(
                    client,
                    &test.queries_path,
                    &test.destination,
                    CodegenSettings::from(&test),
                )
                .map_err(Error::report)?;
                // Format the generated crate
                rustfmt_lib(&test.destination);
            } else {
                let tmp_path = tmp_dir.path().join(
                    test.destination
                        .file_name()
                        .unwrap_or("cornucopia".as_ref()),
                );
                std::fs::create_dir(&tmp_path)?;
                // Generate
                cornucopia::gen_live(
                    client,
                    &test.queries_path,
                    &tmp_path,
                    CodegenSettings::from(&test),
                )
                .map_err(Error::report)?;
                // Format the generated crate
                rustfmt_lib(&tmp_path);

                // If the newly generated crate differs from
                // the currently checked in one, return an error.
                if dir_diff::is_different(&test.destination, &tmp_path).unwrap() {
                    Err(format!(
                        "\"{}\" is outdated",
                        test.destination.to_str().unwrap()
                    ))?;
                }
            }
            println!("(generate) {} {}", test.name, "OK".green());

            if test.run {
                // Change current directory
                std::env::set_current_dir(&original_pwd)?;
                std::env::set_current_dir(&format!("../{}", test.base_path))?;
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
