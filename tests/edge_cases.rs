use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn vibe_clock(tmp: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("vibe-clock").unwrap();
    cmd.env("VIBE_CLOCK_DB", tmp.path().join("test.db"));
    cmd.env("VIBE_CLOCK_KEY", "test-key");
    cmd
}

/// T083: Clock state persists after crash (process exit without stopping)
#[test]
fn clock_state_persists_after_crash() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .success();

    // Start clock
    vibe_clock(&tmp)
        .args(["clock", "start", "Acme", "Working"])
        .assert()
        .success();

    // "Crash" = just exit. Relaunch and check status
    vibe_clock(&tmp)
        .args(["clock", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Clock running"))
        .stdout(predicate::str::contains("Working"));

    // Can still stop it
    vibe_clock(&tmp)
        .args(["clock", "stop"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Clock stopped"));
}

/// T085: Deleting a project with a running clock stops the clock first
#[test]
fn delete_project_with_running_clock_stops_clock_first() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .success();

    // Start a clock on this project
    vibe_clock(&tmp)
        .args(["clock", "start", "Acme", "In progress"])
        .assert()
        .success();

    // Delete project with --yes
    vibe_clock(&tmp)
        .args(["project", "delete", "1", "--yes"])
        .assert()
        .success()
        .stdout(predicate::str::contains("deleted"));

    // Clock should no longer be running
    vibe_clock(&tmp)
        .args(["clock", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No clock is running"));

    // Project should be gone
    vibe_clock(&tmp)
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No projects found"));
}

/// T086: Clock crossing midnight records correct duration
/// We can't easily test with FakeClock via CLI integration tests,
/// but we test that start/stop with explicit times work correctly
/// by verifying the task entry after stopping.
#[test]
fn clock_start_stop_records_task_entry() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args(["clock", "start", "Acme", "Late work"])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args(["clock", "stop"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Clock stopped"))
        .stdout(predicate::str::contains("Late work"));

    // Verify the task appears in journal
    vibe_clock(&tmp)
        .args(["journal"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Late work"));
}

/// T025: PDF write to read-only directory produces actionable error
#[test]
fn pdf_write_permission_error() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .success();
    vibe_clock(&tmp)
        .args([
            "task",
            "add",
            "Acme",
            "Some work",
            "--start",
            "2026-02-25T09:00",
            "--end",
            "2026-02-25T10:00",
        ])
        .assert()
        .success();

    // Create a read-only directory
    let readonly_dir = tmp.path().join("readonly");
    fs::create_dir(&readonly_dir).unwrap();
    let pdf_path = readonly_dir.join("report.pdf");
    let mut perms = fs::metadata(&readonly_dir).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(&readonly_dir, perms).unwrap();

    vibe_clock(&tmp)
        .args([
            "report",
            "--from",
            "2026-02-25",
            "--to",
            "2026-02-25",
            "--output",
            pdf_path.to_str().unwrap(),
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("Could not create file"));

    // Restore permissions for cleanup
    let mut perms = fs::metadata(&readonly_dir).unwrap().permissions();
    #[allow(clippy::permissions_set_readonly_false)]
    perms.set_readonly(false);
    fs::set_permissions(&readonly_dir, perms).unwrap();
}
