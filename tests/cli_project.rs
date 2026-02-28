use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn vibe_clock(tmp: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("vibe-clock").unwrap();
    cmd.env("VIBE_CLOCK_DB", tmp.path().join("test.db"));
    cmd.env("VIBE_CLOCK_KEY", "test-key");
    cmd
}

#[test]
fn creates_project_and_appears_in_list() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Project 'Acme' created."));

    vibe_clock(&tmp)
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Acme"));
}

#[test]
fn rejects_duplicate_project_name() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .code(1)
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn edits_project_name() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args(["project", "edit", "1", "--name", "Beta"])
        .assert()
        .success()
        .stdout(predicate::str::contains("renamed to 'Beta'"));

    vibe_clock(&tmp)
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Beta"))
        .stdout(predicate::str::contains("Acme").not());
}

#[test]
fn deletes_project_without_tasks() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args(["project", "delete", "1", "--yes"])
        .assert()
        .success()
        .stdout(predicate::str::contains("deleted"));

    vibe_clock(&tmp)
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No projects found"));
}

#[test]
fn deletes_project_with_tasks_cascades_with_yes() {
    let tmp = TempDir::new().unwrap();

    // Create project
    vibe_clock(&tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .success();

    // Add a task entry directly via task add (duration-only)
    vibe_clock(&tmp)
        .args(["task", "add", "Acme", "Some work", "--duration", "60"])
        .assert()
        .success();

    // Delete with --yes should cascade
    vibe_clock(&tmp)
        .args(["project", "delete", "1", "--yes"])
        .assert()
        .success()
        .stdout(predicate::str::contains("deleted"));

    // Project should be gone
    vibe_clock(&tmp)
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No projects found"));
}

#[test]
fn lists_empty_projects_with_help_message() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No projects found"));
}
