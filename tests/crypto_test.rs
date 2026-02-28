use tempfile::TempDir;
use vibe_clock::db::Database;

#[test]
fn opens_db_with_env_key() {
    let tmp_dir = TempDir::new().unwrap();
    let db_path = tmp_dir.path().join("encrypted.db");

    // Open with encryption key
    let db = Database::open(&db_path, "test-passphrase-123").unwrap();

    // Write a project
    db.conn
        .execute(
            "INSERT INTO project (name, created_at, updated_at) VALUES (?1, ?2, ?3)",
            rusqlite::params!["Acme", "2026-01-01T00:00:00", "2026-01-01T00:00:00"],
        )
        .unwrap();

    // Close and reopen with same key
    drop(db);
    let db2 = Database::open(&db_path, "test-passphrase-123").unwrap();

    // Read back and verify
    let name: String = db2
        .conn
        .query_row("SELECT name FROM project WHERE id = 1", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert_eq!(name, "Acme");
}

#[test]
fn rejects_wrong_key() {
    let tmp_dir = TempDir::new().unwrap();
    let db_path = tmp_dir.path().join("encrypted.db");

    // Create DB with key A
    let db = Database::open(&db_path, "correct-key").unwrap();
    db.conn
        .execute(
            "INSERT INTO project (name, created_at, updated_at) VALUES (?1, ?2, ?3)",
            rusqlite::params!["Acme", "2026-01-01T00:00:00", "2026-01-01T00:00:00"],
        )
        .unwrap();
    drop(db);

    // Try opening with key B — should fail
    let result = Database::open(&db_path, "wrong-key");
    assert!(result.is_err());
}
