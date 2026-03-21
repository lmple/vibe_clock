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

// --- US1 Tests: Clock Time Start/End + --date ---

#[test]
fn adds_task_with_clock_time_start_end() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args([
            "task", "add", "Acme", "Standup", "--start", "9:00", "--end", "10:30",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("1h 30m"));
}

#[test]
fn adds_task_with_date_and_clock_time() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args([
            "task",
            "add",
            "Acme",
            "Past meeting",
            "--start",
            "9:00",
            "--end",
            "10:30",
            "--date",
            "2026-03-01",
        ])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args(["journal", "2026-03-01"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Past meeting"));
}

#[test]
fn adds_task_date_shortcut_yesterday() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args([
            "task",
            "add",
            "Acme",
            "Yesterday task",
            "--duration",
            "30",
            "--date",
            "yesterday",
        ])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args(["journal", "yesterday"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Yesterday task"));
}

#[test]
fn rejects_iso_datetime_for_start() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args([
            "task",
            "add",
            "Acme",
            "Bad format",
            "--start",
            "2026-03-01T09:00",
            "--end",
            "10:30",
        ])
        .assert()
        .code(1)
        .stderr(predicate::str::contains("Invalid time"));
}

#[test]
fn rejects_equal_start_end_time() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args([
            "task",
            "add",
            "Acme",
            "Zero duration",
            "--start",
            "14:00",
            "--end",
            "14:00",
        ])
        .assert()
        .code(1)
        .stderr(predicate::str::contains(
            "End time must be after start time",
        ));
}

#[test]
fn rejects_midnight_spanning_entry() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args([
            "task",
            "add",
            "Acme",
            "Night shift",
            "--start",
            "23:00",
            "--end",
            "01:00",
        ])
        .assert()
        .code(1)
        .stderr(predicate::str::contains(
            "End time must be after start time",
        ));
}

#[test]
fn rejects_start_time_without_colon() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args([
            "task", "add", "Acme", "Bad time", "--start", "9", "--end", "10:00",
        ])
        .assert()
        .code(1)
        .stderr(predicate::str::contains("Invalid time"));
}

#[test]
fn edits_task_with_date_flag() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    // Add task for today
    vibe_clock(&tmp)
        .args(["task", "add", "Acme", "Move me", "--duration", "30"])
        .assert()
        .success();

    // Move it to yesterday
    vibe_clock(&tmp)
        .args(["task", "edit", "1", "--date", "yesterday"])
        .assert()
        .success();

    // Should appear in yesterday's journal
    vibe_clock(&tmp)
        .args(["journal", "yesterday"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Move me"));

    // Should not appear in today's journal
    vibe_clock(&tmp)
        .args(["journal"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No tasks logged"));
}

#[test]
fn edits_task_start_end_uses_existing_date() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    // Add task for a specific past date
    vibe_clock(&tmp)
        .args([
            "task",
            "add",
            "Acme",
            "Past work",
            "--start",
            "9:00",
            "--end",
            "10:00",
            "--date",
            "2026-03-01",
        ])
        .assert()
        .success();

    // Edit end time only (no --date) — should extend duration, keep date
    vibe_clock(&tmp)
        .args(["task", "edit", "1", "--end", "11:00"])
        .assert()
        .success();

    // Should still appear on that date with updated duration
    vibe_clock(&tmp)
        .args(["journal", "2026-03-01"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Past work"))
        .stdout(predicate::str::contains("2h"));
}

// --- US2 Tests: Human-Friendly Duration ---

#[test]
fn adds_task_with_human_duration() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args(["task", "add", "Acme", "Long work", "--duration", "1h30m"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1h 30m"));
}

#[test]
fn adds_task_with_hours_only_duration() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args(["task", "add", "Acme", "Deep work", "--duration", "2h"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2h"));
}

#[test]
fn adds_task_with_uppercase_duration() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args(["task", "add", "Acme", "Case test", "--duration", "1H30M"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1h 30m"));
}

#[test]
fn rejects_spaced_duration() {
    let tmp = TempDir::new().unwrap();
    setup_project(&tmp);

    vibe_clock(&tmp)
        .args(["task", "add", "Acme", "Bad dur", "--duration", "1h 30m"])
        .assert()
        .code(1)
        .stderr(predicate::str::contains("Invalid duration"));
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
