use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: no text provided"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echo")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let args = &["Hello there"];
    run(args, "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    let args = &["Hello", "there"];
    run(args, "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    let args = &["Hello  there", "-n"];
    run(args, "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    let args = &["-n", "Hello", "there"];
    run(args, "tests/expected/hello2.n.txt")
}
