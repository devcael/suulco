use rusqlite::{Connection, Result as SqliteResult};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

pub struct AppState {
    pub db: Mutex<Connection>,
    pub db_path: PathBuf,
}

const MIGRATIONS: &[(&str, &str)] = &[
    ("001_initial", include_str!("migration/001_initial.sql")),
    ("002_nullable_date", include_str!("migration/002_nullable_date.sql")),
];

pub fn init(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir");
    let db_dir = app_data_dir.join("db");
    std::fs::create_dir_all(&db_dir).ok();

    let db_path = db_dir.join("suulco.sqlite");
    let conn = Connection::open(&db_path)?;

    conn.execute_batch("PRAGMA journal_mode=WAL;")?;
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;

    run_migrations(&conn)?;

    app.manage(AppState {
        db: Mutex::new(conn),
        db_path,
    });

    Ok(())
}

fn run_migrations(conn: &Connection) -> SqliteResult<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _migrations (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            name       TEXT NOT NULL UNIQUE,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        );",
    )?;

    for (name, sql) in MIGRATIONS {
        let already_applied: bool = conn.query_row(
            "SELECT COUNT(*) > 0 FROM _migrations WHERE name = ?1",
            [name],
            |row| row.get(0),
        )?;

        if !already_applied {
            conn.execute_batch("BEGIN;")?;
            let result = conn.execute_batch(sql).and_then(|_| {
                conn.execute("INSERT INTO _migrations (name) VALUES (?1)", [name])
                    .map(|_| ())
            });
            match result {
                Ok(_) => conn.execute_batch("COMMIT;")?,
                Err(e) => {
                    conn.execute_batch("ROLLBACK;").ok();
                    return Err(e);
                }
            }
        }
    }

    Ok(())
}
