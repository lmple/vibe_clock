use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn vibe_clock(tmp: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("vibe-clock").unwrap();
    cmd.env("VIBE_CLOCK_DB", tmp.path().join("test.db"));
    cmd.env("VIBE_CLOCK_KEY", "test-key");
    cmd
}

fn setup_with_tasks(tmp: &TempDir) {
    // Create projects
    vibe_clock(tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .success();
    vibe_clock(tmp)
        .args(["project", "add", "Beta"])
        .assert()
        .success();

    // Add tasks for today
    vibe_clock(tmp)
        .args(["task", "add", "Acme", "Morning standup", "--duration", "30"])
        .assert()
        .success();
    vibe_clock(tmp)
        .args([
            "task",
            "add",
            "Acme",
            "Code review",
            "--start",
            "10:00",
            "--end",
            "11:30",
        ])
        .assert()
        .success();
    vibe_clock(tmp)
        .args(["task", "add", "Beta", "Design meeting", "--duration", "60"])
        .assert()
        .success();
}

#[test]
fn shows_todays_tasks_by_default() {
    let tmp = TempDir::new().unwrap();
    setup_with_tasks(&tmp);

    vibe_clock(&tmp)
        .args(["journal"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Morning standup"))
        .stdout(predicate::str::contains("Code review"))
        .stdout(predicate::str::contains("Design meeting"));
}

#[test]
fn shows_tasks_for_specific_date() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .success();

    // Add task with specific start/end date
    vibe_clock(&tmp)
        .args([
            "task",
            "add",
            "Acme",
            "Old task",
            "--start",
            "2026-02-25T09:00",
            "--end",
            "2026-02-25T10:00",
        ])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args(["journal", "2026-02-25"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Old task"));
}

#[test]
fn shows_empty_message_for_date_with_no_tasks() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["journal", "2026-01-01"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No tasks logged"));
}

#[test]
fn shows_per_project_totals_and_grand_total() {
    let tmp = TempDir::new().unwrap();
    setup_with_tasks(&tmp);

    let output = vibe_clock(&tmp)
        .args(["journal"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(output).unwrap();

    assert!(stdout.contains("Totals:"));
    assert!(stdout.contains("Acme"));
    assert!(stdout.contains("Beta"));
    assert!(stdout.contains("TOTAL"));
}
