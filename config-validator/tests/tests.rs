use std::{fs, io};
use std::io::Write;
use std::process::Command;
use assert_cmd::prelude::*;
use predicates::str::contains;
use tempfile;
use tempfile::NamedTempFile;

// Verify settings-validator exits with a non-zero exit code when no args are provided.
#[test]
fn cli_no_args() {
    Command::cargo_bin("config-validator").unwrap().assert().failure();
}

#[test]
fn cli_version() {
    Command::cargo_bin("settings-validator")
        .unwrap()
        .args(&["-V"])
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn cli_no_config() {
    Command::cargo_bin("config-validator")
        .unwrap()
        .args(&["--keys=key1,key2","--output=output.file"])
        .assert()
        .stderr(contains("--config"));
}

#[test]
fn cli_no_keys() {
    Command::cargo_bin("config-validator")
        .unwrap()
        .args(&["--config=my.config","--output=output.file"])
        .assert()
        .stderr(contains("--keys"));
}

#[test]
fn cli_no_output() {
    Command::cargo_bin("config-validator")
        .unwrap()
        .args(&["--config=my.config","--keys=key1,key2"])
        .assert()
        .stderr(contains("--output"));
}

#[test]
fn keys_exist() -> io::Result<()> {
    run_case("key1=value1\nkey2=value2", true)
}

#[test]
fn keys_exist_whitespace() -> io::Result<()> {
    run_case("    key1=value1\nkey2   =value2", true)
}

#[test]
fn keys_no_exist() -> io::Result<()> {
    run_case("key3=key3", false)
}

fn run_case(config: &str, file_exists: bool) -> io::Result<()> {
    let config_file = setup_config(config)?;
    let config_arg = format!("--config={}", config_file.path().to_str().unwrap());

    let output_dir = tempfile::tempdir()?;
    let output_file = format!("{}/output.file", output_dir.path().to_str().unwrap());
    let output_arg = format!("--output={}", output_file);

    Command::cargo_bin("config-validator")
        .unwrap()
        .args(&[config_arg, output_arg, "--keys=key1,key2".to_string()])
        .assert()
        .success();

    assert_eq!(file_exists, fs::metadata(output_file).is_ok());

    Ok(())
}

fn setup_config(config: &str) -> io::Result<NamedTempFile> {
    let mut config_file = NamedTempFile::new()?;
    config_file.write_all(config.as_bytes())?;

    Ok(config_file)
}

