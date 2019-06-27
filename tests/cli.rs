use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::error::Error;
use std::process::Command; // Run programs

#[test]
fn file_dosent_exist() -> Result<(), Box<Error>> {
    let mut cmd = Command::main_binary()?;
    cmd.arg("Nevermore").arg("./text1");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}

use std::io::{self, Write};
use tempfile::NamedTempFile;

#[test]
fn find_content_in_file() -> Result<(), Box<std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::main_binary()?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}
