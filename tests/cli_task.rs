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

// --- US3 Tests ---

#[test]
fn adds_task_with_start_and_end() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args([
            "task", "add", "Acme", "Meeting", "--start", "09:00", "--end", "10:30",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("1h 30m"));
}

#[test]
fn adds_task_with_duration_only() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args(["task", "add", "Acme", "Quick fix", "--duration", "45"])
        .assert()
        .success()
        .stdout(predicate::str::contains("45m"));
}

#[test]
fn rejects_end_before_start() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args([
            "task",
            "add",
            "Acme",
            "Bad times",
            "--start",
            "10:00",
            "--end",
            "09:00",
        ])
        .assert()
        .code(1)
        .stderr(predicate::str::contains(
            "End time must be after start time",
        ));
}

#[test]
fn rejects_nonexistent_project() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["task", "add", "NoSuchProject", "Work", "--duration", "60"])
        .assert()
        .code(1)
        .stderr(predicate::str::contains("not found"));
}

// --- US6 Tests ---

#[test]
fn edits_task_description() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args(["task", "add", "Acme", "Original desc", "--duration", "30"])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args(["task", "edit", "1", "--description", "Updated desc"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Task 1 updated"));

    // Verify in journal
    vibe_clock(&tmp)
        .args(["journal"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Updated desc"));
}

#[test]
fn edits_task_times() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args([
            "task", "add", "Acme", "Work", "--start", "09:00", "--end", "10:00",
        ])
        .assert()
        .success();

    // Edit end time to extend to 11:00
    vibe_clock(&tmp)
        .args(["task", "edit", "1", "--end", "11:00"])
        .assert()
        .success();

    // Verify updated duration shows in journal (should be 2h now)
    vibe_clock(&tmp)
        .args(["journal"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2h"));
}

#[test]
fn moves_task_to_different_project() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args(["project", "add", "Beta"])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args(["task", "add", "Acme", "Some work", "--duration", "60"])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args(["task", "edit", "1", "--project", "Beta"])
        .assert()
        .success();

    // Verify task now shows under Beta in journal
    let output = vibe_clock(&tmp)
        .args(["journal"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(output).unwrap();
    assert!(stdout.contains("Beta"));
}

#[test]
fn deletes_task_with_yes_flag() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args(["task", "add", "Acme", "To delete", "--duration", "30"])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args(["task", "delete", "1", "--yes"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Task 1 deleted"));

    // Verify task is gone
    vibe_clock(&tmp)
        .args(["journal"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No tasks logged"));
}

#[test]
fn rejects_edit_nonexistent_task() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["task", "edit", "999", "--description", "nope"])
        .assert()
        .code(1)
        .stderr(predicate::str::contains("not found"));
}
