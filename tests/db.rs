use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_output_of_running_db() -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::cargo_bin("sqlite-clone")?;

    command.assert().stdout("db > ");

    Ok(())
}