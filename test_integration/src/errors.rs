use cornucopia::{CodegenSettings, Error};
use owo_colors::OwoColorize;

use crate::{
    fixtures::{ErrorTest, TestSuite},
    utils::reset_db,
};

/// Run errors test, return true if all test are successful
pub(crate) fn run_errors_test(
    client: &mut postgres::Client,
    apply: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut successful = true;
    let original_pwd = std::env::current_dir()?;
    let test_suites = TestSuite::<ErrorTest>::read("fixtures/errors");
    let tmp = tempfile::tempdir()?;

    for mut suite in test_suites {
        println!("{} {}", "[error]".magenta(), suite.name.magenta());
        for test in suite.tests.iter_mut() {
            // Reset db
            reset_db(client)?;

            // Generate file tree path
            let temp_dir = tempfile::tempdir()?;

            // We need to change current dir for error path to always be the same
            std::env::set_current_dir(&temp_dir)?;

            // Generate schema
            std::fs::write(
                "schema.sql",
                [
                    "CREATE TABLE author (id SERIAL, name TEXT);\n",
                    test.schema.as_deref().unwrap_or_default(),
                ]
                .concat(),
            )?;

            // Generate queries files
            std::fs::create_dir("queries")?;
            std::fs::write(
                "queries/test.sql",
                test.query.as_deref().unwrap_or_default(),
            )?;

            // Run codegen
            let result = cornucopia::load_schema(client, &["schema.sql"])
                .map_err(Error::from)
                .and_then(|_| {
                    cornucopia::gen_live(
                        client,
                        "queries".as_ref(),
                        tmp.path(),
                        CodegenSettings::from(&*test),
                    )
                });

            let err = result.unwrap_err().report();
            let err_trimmed = err.trim();
            if err_trimmed == test.error.trim() {
                println!("{} {}", test.name, "OK".green());
            } else {
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
                successful = false;
                println!(
                    "{} {}\n{}\n{}\n{}\n{}\n",
                    test.name,
                    "ERR".red(),
                    expected_msg,
                    test.error,
                    got_msg,
                    err,
                );
            }
            if apply {
                test.error = err_trimmed.into();
            }
            std::env::set_current_dir(&original_pwd)?;
        }

        // Update error message if needed
        if apply {
            suite.write()?;
        }
    }

    Ok(successful)
}
