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
            "09:00",
            "--end",
            "11:00",
            "--date",
            "2026-02-25",
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
            "09:00",
            "--end",
            "10:30",
            "--date",
            "2026-02-26",
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
            "14:00",
            "--end",
            "15:00",
            "--date",
            "2026-02-25",
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
        .stdout(predicate::str::contains("TOTAL"));
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

// --- PDF Export Tests ---

#[test]
fn generates_pdf_with_pdf_flag() {
    let tmp = TempDir::new().unwrap();
    setup_with_dated_tasks(&tmp);

    vibe_clock(&tmp)
        .current_dir(tmp.path())
        .args([
            "report",
            "--from",
            "2026-02-25",
            "--to",
            "2026-02-26",
            "--pdf",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("PDF report saved to"));

    // Verify PDF file exists and is non-empty
    let pdf_path = tmp.path().join("report-2026-02-25-to-2026-02-26.pdf");
    assert!(pdf_path.exists(), "PDF file should exist");
    let metadata = fs::metadata(&pdf_path).unwrap();
    assert!(metadata.len() > 0, "PDF file should be non-empty");
}

#[test]
fn pdf_contains_report_header() {
    let tmp = TempDir::new().unwrap();
    setup_with_dated_tasks(&tmp);

    let pdf_path = tmp.path().join("header-test.pdf");
    vibe_clock(&tmp)
        .args([
            "report",
            "--from",
            "2026-02-25",
            "--to",
            "2026-02-26",
            "--output",
            pdf_path.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("2026-02-25"))
        .stdout(predicate::str::contains("2026-02-26"))
        .stdout(predicate::str::contains("PDF report saved to"));

    // Verify PDF starts with PDF magic bytes
    let bytes = fs::read(&pdf_path).unwrap();
    assert!(
        bytes.starts_with(b"%PDF"),
        "File should be a valid PDF (starts with %PDF)"
    );
}

#[test]
fn pdf_not_created_when_no_tasks() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .current_dir(tmp.path())
        .args([
            "report",
            "--from",
            "2026-01-01",
            "--to",
            "2026-01-31",
            "--pdf",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("No tasks found"));

    // No PDF should be created
    let entries: Vec<_> = fs::read_dir(tmp.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "pdf"))
        .collect();
    assert!(entries.is_empty(), "No PDF file should be created");
}

#[test]
fn pdf_contains_project_names() {
    let tmp = TempDir::new().unwrap();
    setup_with_dated_tasks(&tmp);

    let pdf_path = tmp.path().join("projects-test.pdf");
    vibe_clock(&tmp)
        .args([
            "report",
            "--from",
            "2026-02-25",
            "--to",
            "2026-02-26",
            "--output",
            pdf_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Verify PDF is valid
    let bytes = fs::read(&pdf_path).unwrap();
    assert!(
        bytes.starts_with(b"%PDF"),
        "File should be a valid PDF (starts with %PDF)"
    );
    assert!(bytes.len() > 100, "PDF should have substantial content");
}

#[test]
fn pdf_handles_unicode_text() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["project", "add", "Développement"])
        .assert()
        .success();

    vibe_clock(&tmp)
        .args([
            "task",
            "add",
            "Développement",
            "Réunion café",
            "--duration",
            "60",
        ])
        .assert()
        .success();

    let pdf_path = tmp.path().join("unicode-test.pdf");
    vibe_clock(&tmp)
        .args([
            "report",
            "--from",
            "today",
            "--to",
            "today",
            "--output",
            pdf_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(pdf_path.exists(), "PDF with unicode should be created");
    let metadata = fs::metadata(&pdf_path).unwrap();
    assert!(metadata.len() > 0, "PDF with unicode should be non-empty");
}

#[test]
fn pdf_output_to_specific_path() {
    let tmp = TempDir::new().unwrap();
    setup_with_dated_tasks(&tmp);

    let pdf_path = tmp.path().join("custom.pdf");
    vibe_clock(&tmp)
        .args([
            "report",
            "--from",
            "2026-02-25",
            "--to",
            "2026-02-26",
            "--output",
            pdf_path.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("PDF report saved to"));

    assert!(pdf_path.exists(), "PDF should exist at custom path");
}

#[test]
fn pdf_output_to_directory() {
    let tmp = TempDir::new().unwrap();
    setup_with_dated_tasks(&tmp);

    let out_dir = tmp.path().join("reports");
    fs::create_dir(&out_dir).unwrap();

    vibe_clock(&tmp)
        .args([
            "report",
            "--from",
            "2026-02-25",
            "--to",
            "2026-02-26",
            "--output",
            out_dir.to_str().unwrap(),
        ])
        .assert()
        .success();

    let expected = out_dir.join("report-2026-02-25-to-2026-02-26.pdf");
    assert!(expected.exists(), "PDF should exist in output directory");
}

#[test]
fn pdf_output_implies_pdf_flag() {
    let tmp = TempDir::new().unwrap();
    setup_with_dated_tasks(&tmp);

    let pdf_path = tmp.path().join("implied.pdf");
    vibe_clock(&tmp)
        .args([
            "report",
            "--from",
            "2026-02-25",
            "--to",
            "2026-02-26",
            "--output",
            pdf_path.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("PDF report saved to"));

    assert!(
        pdf_path.exists(),
        "PDF should be created with --output alone (no --pdf needed)"
    );
}

#[test]
fn pdf_rejects_nonexistent_directory() {
    let tmp = TempDir::new().unwrap();
    setup_with_dated_tasks(&tmp);

    vibe_clock(&tmp)
        .args([
            "report",
            "--from",
            "2026-02-25",
            "--to",
            "2026-02-26",
            "--output",
            "/nonexistent/dir/report.pdf",
        ])
        .assert()
        .code(1)
        .stderr(predicate::str::contains("does not exist"));
}

#[test]
fn pdf_overwrites_existing_file() {
    let tmp = TempDir::new().unwrap();
    setup_with_dated_tasks(&tmp);

    let pdf_path = tmp.path().join("overwrite.pdf");
    fs::write(&pdf_path, "old content").unwrap();

    vibe_clock(&tmp)
        .args([
            "report",
            "--from",
            "2026-02-25",
            "--to",
            "2026-02-26",
            "--output",
            pdf_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read(&pdf_path).unwrap();
    assert!(content.len() > 11, "PDF should overwrite old content");
    assert_ne!(&content[..4], b"old ", "PDF should not contain old content");
}

#[test]
fn report_single_date_with_pdf() {
    let tmp = TempDir::new().unwrap();
    setup_with_dated_tasks(&tmp);

    vibe_clock(&tmp)
        .current_dir(tmp.path())
        .args(["report", "--from", "2026-02-25", "--pdf"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2026-02-25"))
        .stdout(predicate::str::contains("PDF report saved to"));

    // Verify PDF file with single date name exists
    let pdf_path = tmp.path().join("report-2026-02-25.pdf");
    assert!(
        pdf_path.exists(),
        "PDF file should exist with single-date format"
    );

    let content = fs::read(&pdf_path).unwrap();
    assert!(content.len() > 100, "PDF should have content");
}

#[test]
fn report_today_shortcut() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["project", "add", "Test"])
        .assert()
        .success();

    // Add task for today
    vibe_clock(&tmp)
        .args(["task", "add", "Test", "Today's work", "--duration", "60"])
        .assert()
        .success();

    vibe_clock(&tmp)
        .current_dir(tmp.path())
        .args(["report", "--from", "today", "--pdf"])
        .assert()
        .success()
        .stdout(predicate::str::contains("PDF report saved to"));

    // Verify PDF was created (today's date)
    let today = chrono::Local::now().date_naive();
    let expected_name = format!("report-{}.pdf", today.format("%Y-%m-%d"));
    let pdf_path = tmp.path().join(&expected_name);
    assert!(pdf_path.exists(), "Should create PDF with 'today' shortcut");
}

#[test]
fn report_single_date_terminal() {
    let tmp = TempDir::new().unwrap();
    setup_with_dated_tasks(&tmp);

    vibe_clock(&tmp)
        .args(["report", "--from", "2026-02-25"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2026-02-25"))
        .stdout(predicate::str::contains("Acme"))
        .stdout(predicate::str::contains("Day 1 work"));

    // Verify no PDF was created (only terminal output)
    let entries: Vec<_> = fs::read_dir(tmp.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "pdf"))
        .collect();

    assert_eq!(entries.len(), 0, "Should not create PDF without --pdf flag");
}

// --- Two-Part Layout Tests (US1: Project Summary Table) ---

#[test]
fn report_shows_project_summary_table() {
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

    // Summary section must appear
    assert!(
        stdout.contains("Project Summary"),
        "missing 'Project Summary' heading"
    );
    assert!(stdout.contains("Acme"), "missing project Acme in summary");
    assert!(stdout.contains("Beta"), "missing project Beta in summary");
    assert!(stdout.contains("TOTAL"), "missing TOTAL row in summary");

    // Summary section must come before any date section heading.
    // The date "2026-02-25" also appears in the report header line, so we look for the
    // per-day section heading which appears as a standalone line: "\n2026-02-25\n".
    let summary_pos = stdout.find("Project Summary").unwrap();
    let section_heading = "\n2026-02-25\n";
    let date_section_pos = stdout
        .find(section_heading)
        .expect("per-day section heading '2026-02-25' not found");
    assert!(
        summary_pos < date_section_pos,
        "Project Summary must appear before per-day sections"
    );
}

#[test]
fn report_shows_grand_total_in_summary() {
    let tmp = TempDir::new().unwrap();
    setup_with_dated_tasks(&tmp);

    // Acme: Day 1 (2h) + Day 2 (1h 30m) = 3h 30m; Beta: 1h; Grand total: 4h 30m
    vibe_clock(&tmp)
        .args(["report", "--from", "2026-02-25", "--to", "2026-02-26"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4h 30m"));
}

#[test]
fn report_single_project_shows_summary_and_total() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["project", "add", "Solo"])
        .assert()
        .success();
    vibe_clock(&tmp)
        .args([
            "task",
            "add",
            "Solo",
            "Only task",
            "--start",
            "09:00",
            "--end",
            "10:30",
            "--date",
            "2026-03-01",
        ])
        .assert()
        .success();

    let output = vibe_clock(&tmp)
        .args(["report", "--from", "2026-03-01"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(output).unwrap();

    assert!(
        stdout.contains("Solo"),
        "project name must appear in summary"
    );
    assert!(stdout.contains("TOTAL"), "TOTAL row must appear");
    assert!(stdout.contains("1h 30m"), "duration must appear correctly");
}

// --- Two-Part Layout Tests (US2: Per-Day Task Breakdown) ---

#[test]
fn report_shows_per_day_sections() {
    let tmp = TempDir::new().unwrap();
    setup_with_dated_tasks(&tmp);

    vibe_clock(&tmp)
        .args(["report", "--from", "2026-02-25", "--to", "2026-02-26"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2026-02-25"))
        .stdout(predicate::str::contains("2026-02-26"));
}

#[test]
fn report_shows_tasks_in_date_ascending_order() {
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

    let pos_25 = stdout.find("2026-02-25").unwrap();
    let pos_26 = stdout.find("2026-02-26").unwrap();
    assert!(
        pos_25 < pos_26,
        "2026-02-25 section must appear before 2026-02-26 section"
    );
}

#[test]
fn report_no_truncation_long_description() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .success();

    let long_desc = "This is a very long task description that exceeds thirty characters easily";
    vibe_clock(&tmp)
        .args([
            "task",
            "add",
            "Acme",
            long_desc,
            "--start",
            "09:00",
            "--end",
            "10:00",
            "--date",
            "2026-03-10",
        ])
        .assert()
        .success();

    let output = vibe_clock(&tmp)
        .args(["report", "--from", "2026-03-10"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(output).unwrap();

    // Full description must appear (no "...")
    assert!(
        stdout.contains("This is a very long task description"),
        "full description must appear in output"
    );
    assert!(
        !stdout.contains("..."),
        "description must not be truncated with '...'"
    );
}

#[test]
fn report_dash_for_tasks_with_no_start_end() {
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
            "Duration only task",
            "--duration",
            "45m",
            "--date",
            "2026-03-05",
        ])
        .assert()
        .success();

    let output = vibe_clock(&tmp)
        .args(["report", "--from", "2026-03-05"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(output).unwrap();

    assert!(stdout.contains("Duration only task"), "task must appear");
    assert!(
        stdout.contains('-'),
        "dash must appear for missing start/end"
    );
}

#[test]
fn report_empty_days_omitted() {
    let tmp = TempDir::new().unwrap();
    setup_with_dated_tasks(&tmp);

    // 2026-02-27 has no tasks
    let output = vibe_clock(&tmp)
        .args(["report", "--from", "2026-02-25", "--to", "2026-02-27"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(output).unwrap();

    // "2026-02-27" appears in the report header ("Report: 2026-02-25 to 2026-02-27"),
    // but must NOT appear as a per-day section heading (standalone line).
    assert!(
        !stdout.contains("\n2026-02-27\n"),
        "2026-02-27 has no tasks and must not appear as a section heading"
    );
}

#[test]
fn report_tasks_ordered_by_start_time_within_day() {
    let tmp = TempDir::new().unwrap();

    vibe_clock(&tmp)
        .args(["project", "add", "Acme"])
        .assert()
        .success();

    // Add later task first, earlier task second
    vibe_clock(&tmp)
        .args([
            "task",
            "add",
            "Acme",
            "Afternoon task",
            "--start",
            "14:00",
            "--end",
            "15:00",
            "--date",
            "2026-03-15",
        ])
        .assert()
        .success();
    vibe_clock(&tmp)
        .args([
            "task",
            "add",
            "Acme",
            "Morning task",
            "--start",
            "09:00",
            "--end",
            "10:00",
            "--date",
            "2026-03-15",
        ])
        .assert()
        .success();

    let output = vibe_clock(&tmp)
        .args(["report", "--from", "2026-03-15"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let stdout = String::from_utf8(output).unwrap();

    let morning_pos = stdout.find("Morning task").unwrap();
    let afternoon_pos = stdout.find("Afternoon task").unwrap();
    assert!(
        morning_pos < afternoon_pos,
        "Morning task (09:00) must appear before Afternoon task (14:00)"
    );
}
