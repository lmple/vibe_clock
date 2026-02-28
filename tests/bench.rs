use std::time::Instant;
use tempfile::TempDir;
use vibe_clock::db::Database;

/// T089: Performance benchmark - seed database with 2,000 entries and 50 projects,
/// measure journal query time (<200ms) and yearly report generation time (<1s).
#[test]
fn performance_with_2000_entries() {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("bench.db");
    let db = Database::open_unencrypted(&db_path).unwrap();

    // Seed 50 projects
    let base_date = chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    for i in 1..=50 {
        let now = base_date.and_hms_opt(0, 0, 0).unwrap();
        db.insert_project(&format!("Project-{i}"), now).unwrap();
    }

    // Seed 2,000 task entries across projects and dates
    for i in 0..2000 {
        let project_id = (i % 50) as i64 + 1;
        let day_offset = i / 10; // ~200 days of entries
        let date = base_date + chrono::Duration::days(day_offset as i64);
        let start = date.and_hms_opt(9, 0, 0).unwrap();
        let end = date.and_hms_opt(9, 30, 0).unwrap();
        let now = date.and_hms_opt(10, 0, 0).unwrap();
        db.insert_task_entry(
            project_id,
            &format!("Task {i}"),
            Some(start),
            Some(end),
            30,
            now,
        )
        .unwrap();
    }

    // Benchmark: journal query for a single day (<200ms)
    let start = Instant::now();
    let tasks = db.list_tasks_for_date("2026-03-15").unwrap();
    let journal_time = start.elapsed();
    println!("Journal query: {} tasks in {:?}", tasks.len(), journal_time);
    assert!(
        journal_time.as_millis() < 200,
        "Journal query took {:?}, expected <200ms",
        journal_time
    );

    // Benchmark: yearly report (<1s)
    let start = Instant::now();
    let tasks = db
        .list_tasks_for_date_range("2026-01-01", "2026-12-31")
        .unwrap();
    let report_time = start.elapsed();
    println!("Yearly report: {} tasks in {:?}", tasks.len(), report_time);
    assert!(
        report_time.as_millis() < 1000,
        "Yearly report took {:?}, expected <1s",
        report_time
    );
}
