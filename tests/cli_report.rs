use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn vibe_clock(tmp: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("vibe-clock").unwrap();
    cmd.env("VIBE_CLOCK_DB", tmp.path().join("test.db"));
    cmd.env("VIBE_CLOCK_KEY", "test-key");
    cmd
}

fn setup_with_dated_tasks(tmp: &TempDir) {
    vibe_clock(tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .success();
    vibe_clock(tmp)
        .args(["project", "add", "Beta"])
        .assert()
        .success();

    // Tasks on different days
    vibe_clock(tmp)
        .args([
            "task",
            "add",
            "Acme",
            "Day 1 work",
            "--start",
            "2026-02-25T09:00",
            "--end",
            "2026-02-25T11:00",
        ])
        .assert()
        .success();
    vibe_clock(tmp)
        .args([
            "task",
            "add",
            "Acme",
            "Day 2 work",
            "--start",
            "2026-02-26T09:00",
            "--end",
            "2026-02-26T10:30",
        ])
        .assert()
        .success();
    vibe_clock(tmp)
        .args([
            "task",
            "add",
            "Beta",
            "Beta task",
            "--start",
            "2026-02-25T14:00",
            "--end",
            "2026-02-25T15:00",
        ])
        .assert()
        .success();
}

#[test]
fn generates_report_grouped_by_project() {
    let tmp = TempDir::new().unwrap();
    setup_with_dated_tasks(&tmp);

    vibe_clock(&tmp)
        .args(["report", "--from", "2026-02-25", "--to", "2026-02-26"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Acme"))
        .stdout(predicate::str::contains("Beta"))
        .stdout(predicate::str::contains("Grand Total"));
}

#[test]
fn shows_day_by_day_breakdown() {
    let tmp = TempDir::new().unwrap();
    setup_with_dated_tasks(&tmp);

    let output = vibe_clock(&tmp)
        .args(["report", "--from", "2026-02-25", "--to", "2026-02-26"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(output).unwrap();

    assert!(stdout.contains("Day 1 work"));
    assert!(stdout.contains("Day 2 work"));
    assert!(stdout.contains("2026-02-25"));
    assert!(stdout.contains("2026-02-26"));
}

#[test]
fn shows_empty_message_for_range_with_no_data() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["report", "--from", "2026-01-01", "--to", "2026-01-31"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No tasks found"));
}

#[test]
fn rejects_from_after_to() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["report", "--from", "2026-02-28", "--to", "2026-02-01"])
        .assert()
        .code(1)
        .stderr(predicate::str::contains(
            "--from date must be before or equal to --to date",
        ));
}
