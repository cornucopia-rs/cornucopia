use std::{path::Path, process::Command};

/// Reset the current database
pub(crate) fn reset_db(client: &mut postgres::Client) -> Result<(), postgres::Error> {
    client.batch_execute("DROP SCHEMA public CASCADE;CREATE SCHEMA public;")
}

pub(crate) fn rustfmt_lib(path: &Path) {
    assert!(Command::new("rustfmt")
        .args([
            "--edition",
            "2021",
            path.join("src/lib.rs").to_str().unwrap(),
        ])
        .status()
        .unwrap()
        .success());
}
