use chrono::NaiveDateTime;
use tempfile::TempDir;
use vibe_clock::clock_trait::Clock;
use vibe_clock::db::Database;

pub struct FakeClock {
    pub now: NaiveDateTime,
}

impl FakeClock {
    pub fn new(now: NaiveDateTime) -> Self {
        Self { now }
    }
}

impl Clock for FakeClock {
    fn now(&self) -> NaiveDateTime {
        self.now
    }
}

pub fn create_test_db() -> (TempDir, Database) {
    let tmp_dir = TempDir::new().expect("Failed to create temp dir");
    let db_path = tmp_dir.path().join("test.db");
    let db = Database::open_unencrypted(&db_path).expect("Failed to open test database");
    (tmp_dir, db)
}
