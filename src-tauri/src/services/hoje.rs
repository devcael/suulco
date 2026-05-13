use crate::db::AppState;
use serde::Serialize;
use tauri::{command, State};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HojeItem {
    pub id: i32,
    pub text: String,
    pub is_done: bool,
    pub linked_sulco: Option<SulcoRef>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SulcoRef {
    pub id: i32,
    pub text: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HojeSummary {
    pub total: i32,
    pub pending: i32,
    pub done: i32,
}

#[command]
pub fn get_hoje_items(state: State<AppState>) -> Result<Vec<HojeItem>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT h.id, h.text, h.done, s.id, s.text
             FROM hoje_items h
             LEFT JOIN sulco_items s ON h.sulco_id = s.id
             WHERE h.date = date('now')
             ORDER BY h.id ASC",
        )
        .map_err(|e| e.to_string())?;
    let items = stmt
        .query_map([], |row| {
            let sulco_id: Option<i32> = row.get(3)?;
            let sulco_text: Option<String> = row.get(4)?;
            Ok(HojeItem {
                id: row.get(0)?,
                text: row.get(1)?,
                is_done: row.get::<_, i32>(2)? != 0,
                linked_sulco: sulco_id.zip(sulco_text).map(|(id, text)| SulcoRef { id, text }),
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(items)
}

#[command]
pub fn get_hoje_summary(state: State<AppState>) -> Result<HojeSummary, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.query_row(
        "SELECT COUNT(*), SUM(CASE WHEN done = 0 THEN 1 ELSE 0 END), SUM(done)
         FROM hoje_items WHERE date = date('now')",
        [],
        |row| {
            Ok(HojeSummary {
                total: row.get::<_, Option<i32>>(0)?.unwrap_or(0),
                pending: row.get::<_, Option<i32>>(1)?.unwrap_or(0),
                done: row.get::<_, Option<i32>>(2)?.unwrap_or(0),
            })
        },
    )
    .map_err(|e| e.to_string())
}

#[command]
pub fn toggle_task_status(state: State<AppState>, id: i32, is_done: bool) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE hoje_items SET done = ?1 WHERE id = ?2",
        rusqlite::params![is_done as i32, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn delete_task(state: State<AppState>, id: i32) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM hoje_items WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn link_task_to_sulco(
    state: State<AppState>,
    task_id: i32,
    sulco_id: Option<i32>,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE hoje_items SET sulco_id = ?1 WHERE id = ?2",
        rusqlite::params![sulco_id, task_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn update_task_text(state: State<AppState>, id: i32, text: String) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE hoje_items SET text = ?1 WHERE id = ?2",
        rusqlite::params![text, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn defer_task_to_inbox(state: State<AppState>, id: i32) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE hoje_items SET date = NULL WHERE id = ?1",
        [id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn create_task_linked_to_sulco(state: State<AppState>, text: String, sulco_id: i32) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO hoje_items (text, date, sulco_id, done) VALUES (?1, date('now'), ?2, 0)",
        rusqlite::params![text, sulco_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
