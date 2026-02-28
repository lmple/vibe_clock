use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn vibe_clock(tmp: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("vibe-clock").unwrap();
    cmd.env("VIBE_CLOCK_DB", tmp.path().join("test.db"));
    cmd.env("VIBE_CLOCK_KEY", "test-key");
    cmd
}

fn setup_project(tmp: &TempDir) {
    vibe_clock(tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .success();
}

#[test]
fn starts_clock_and_shows_confirmation() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args(["clock", "start", "Acme", "Working on feature"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Clock started"));
}

#[test]
fn stops_clock_and_logs_task() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args(["clock", "start", "Acme", "Working on feature"])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args(["clock", "stop"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Clock stopped"));
}

#[test]
fn shows_clock_status_when_running() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args(["clock", "start", "Acme", "Working on feature"])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args(["clock", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Clock running"))
        .stdout(predicate::str::contains("Acme"))
        .stdout(predicate::str::contains("Working on feature"));
}

#[test]
fn rejects_second_clock_start() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args(["clock", "start", "Acme", "First task"])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args(["clock", "start", "Acme", "Second task"])
        .assert()
        .code(1)
        .stderr(predicate::str::contains("already running"));
}

#[test]
fn reports_no_clock_running_on_stop() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args(["clock", "stop"])
        .assert()
        .code(1)
        .stderr(predicate::str::contains("No clock is running"));
}

#[test]
fn recovers_clock_state_after_crash() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    // Start a clock
    vibe_clock(&tmp)
        .args(["clock", "start", "Acme", "Crashed task"])
        .assert()
        .success();

    // Run any command — should show warning about running clock
    vibe_clock(&tmp)
        .args(["clock", "status"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Warning: Clock still running"));

    // Clock can still be stopped
    vibe_clock(&tmp)
        .args(["clock", "stop"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Clock stopped"));
}
