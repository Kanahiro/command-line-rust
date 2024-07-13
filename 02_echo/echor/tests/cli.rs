use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn die_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hoge").assert().success().stdout("hoge\n");
}

#[test]
fn omit_newline() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(&["-n", "hoge"]).assert().success().stdout("hoge");
}

fn run(args: &[&str], expected_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() {
    run(&["Hello there"], "tests/expected/hello1.txt").unwrap();
}

#[test]
fn hello2() {
    run(&["Hello", "there"], "tests/expected/hello2.txt").unwrap();
}

#[test]
fn hello3() {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt").unwrap();
}

#[test]
fn hello4() {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt").unwrap();
}
