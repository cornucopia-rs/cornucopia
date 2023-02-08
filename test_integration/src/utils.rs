use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

/// Reset the current database
pub(crate) fn reset_db(client: &mut postgres::Client) -> Result<(), postgres::Error> {
    client.batch_execute("DROP SCHEMA public CASCADE;CREATE SCHEMA public;")
}

pub(crate) fn rustfmt_file(path: &Path) {
    Command::new("rustfmt")
        .args(["--edition", "2021"])
        .arg(path)
        .output()
        .unwrap();
}

pub(crate) fn rustfmt_string(string: &str) -> String {
    // Format the generated code string by piping to rustfmt
    let mut rustfmt = Command::new("rustfmt")
        .args(["--edition", "2021"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    rustfmt
        .stdin
        .as_mut()
        .unwrap()
        .write_all(string.as_bytes())
        .unwrap();
    String::from_utf8(rustfmt.wait_with_output().unwrap().stdout).unwrap()
}
