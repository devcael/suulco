use crate::db::AppState;
use tauri::{command, State};

#[command]
pub fn create_global_item(
    state: State<AppState>,
    destination: String,
    text: String,
    sulco_id: Option<i32>,
    category: Option<String>,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    match destination.as_str() {
        "hoje" => {
            conn.execute(
                "INSERT INTO hoje_items (text, date, sulco_id) VALUES (?1, date('now'), ?2)",
                rusqlite::params![text, sulco_id],
            )
            .map_err(|e| e.to_string())?;
        }
        "inbox" => {
            conn.execute(
                "INSERT INTO hoje_items (text, date, sulco_id) VALUES (?1, NULL, ?2)",
                rusqlite::params![text, sulco_id],
            )
            .map_err(|e| e.to_string())?;
        }
        "sulco" => {
            conn.execute(
                "INSERT INTO sulco_items (text) VALUES (?1)",
                [&text],
            )
            .map_err(|e| e.to_string())?;
        }
        "memoria" => {
            conn.execute(
                "INSERT INTO memoria_items (text, category) VALUES (?1, ?2)",
                rusqlite::params![text, category],
            )
            .map_err(|e| e.to_string())?;
        }
        _ => return Err(format!("Unknown destination: {}", destination)),
    }

    Ok(())
}
