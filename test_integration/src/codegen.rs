use crate::{
    fixtures::{CodegenTest, TestSuite},
    utils::{reset_db, rustfmt_file, rustfmt_string},
};

use cornucopia::{CodegenSettings, Error};
use owo_colors::OwoColorize;
use std::{env::set_current_dir, process::Command};

// Run codegen test, return true if all test are successful
pub(crate) fn run_codegen_test(
    client: &mut postgres::Client,
    apply: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut successful = true;
    let original_pwd = std::env::current_dir()?;
    let fixture_path = "fixtures/codegen";

    let test_suites = TestSuite::<CodegenTest>::read(fixture_path);
    for suite in test_suites {
        println!("{}", format!("[codegen] {}", suite.name).magenta());
        for test in suite.tests {
            set_current_dir(format!("../{}", test.base_path))?;

            // Load schema
            reset_db(client)?;
            cornucopia::load_schema(client, vec!["schema.sql".to_string()])?;

            // If `--apply`, then the code will be regenerated.
            // Otherwise, it is only checked.
            if apply {
                // Generate
                cornucopia::generate_live(
                    client,
                    test.queries_path.to_str().unwrap(), // TODO: Update this once our API accepts paths
                    Some(test.destination.to_str().unwrap()), // TODO: Update this once our API accepts paths
                    CodegenSettings::from(&test),
                )
                .map_err(Error::report)?;
                // Format the generated file
                rustfmt_file(&test.destination);
            } else {
                // Get currently checked-in generate file
                let old_codegen = std::fs::read_to_string(&test.destination).unwrap();
                // Generate new file
                let new_codegen = cornucopia::generate_live(
                    client,
                    test.queries_path.to_str().unwrap(), // TODO: Update this once our API accepts paths
                    None,
                    CodegenSettings::from(&test),
                )
                .map_err(Error::report)?;
                // Format the generated code string by piping to rustfmt
                let new_codegen_formatted = rustfmt_string(&new_codegen);

                // If the newly generated file differs from
                // the currently checked in one, return an error.
                if old_codegen != new_codegen_formatted {
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
