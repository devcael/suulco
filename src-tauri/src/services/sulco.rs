use crate::db::AppState;
use serde::Serialize;
use tauri::{command, State};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SulcoListItem {
    pub id: i32,
    pub text: String,
    pub is_open: bool,
    pub has_definition: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SulcoDetail {
    pub id: i32,
    pub text: String,
    pub definition: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivedSulcoItem {
    pub id: i32,
    pub text: String,
    pub archived_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SulcoLinkOption {
    pub id: i32,
    pub text: String,
    pub is_currently_linked: bool,
}

#[command]
pub fn get_active_sulco_items(state: State<AppState>) -> Result<Vec<SulcoListItem>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, text, definition FROM sulco_items WHERE archived = 0 ORDER BY position ASC",
        )
        .map_err(|e| e.to_string())?;
    let items = stmt
        .query_map([], |row| {
            let definition: String = row.get(2)?;
            Ok(SulcoListItem {
                id: row.get(0)?,
                text: row.get(1)?,
                is_open: false,
                has_definition: !definition.trim().is_empty(),
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(items)
}

#[command]
pub fn get_sulco_detail(state: State<AppState>, id: i32) -> Result<SulcoDetail, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.query_row(
        "SELECT id, text, definition FROM sulco_items WHERE id = ?1",
        [id],
        |row| {
            Ok(SulcoDetail {
                id: row.get(0)?,
                text: row.get(1)?,
                definition: row.get::<_, Option<String>>(2)?.unwrap_or_default(),
            })
        },
    )
    .map_err(|e| e.to_string())
}

#[command]
pub fn update_sulco_text(state: State<AppState>, id: i32, text: String) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE sulco_items SET text = ?1 WHERE id = ?2",
        rusqlite::params![text, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn update_sulco_definition(
    state: State<AppState>,
    id: i32,
    definition: String,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE sulco_items SET definition = ?1 WHERE id = ?2",
        rusqlite::params![definition, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn archive_sulco_item(state: State<AppState>, id: i32) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE sulco_items SET archived = 1, archived_at = datetime('now') WHERE id = ?1",
        [id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn get_archived_sulco_items(state: State<AppState>) -> Result<Vec<ArchivedSulcoItem>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, text, archived_at FROM sulco_items WHERE archived = 1 ORDER BY archived_at DESC",
        )
        .map_err(|e| e.to_string())?;
    let items = stmt
        .query_map([], |row| {
            Ok(ArchivedSulcoItem {
                id: row.get(0)?,
                text: row.get(1)?,
                archived_at: row.get::<_, Option<String>>(2)?.unwrap_or_default(),
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(items)
}

#[command]
pub fn restore_sulco_item(state: State<AppState>, id: i32) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE sulco_items SET archived = 0, archived_at = NULL WHERE id = ?1",
        [id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn get_sulco_link_options(
    state: State<AppState>,
    task_id: i32,
) -> Result<Vec<SulcoLinkOption>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let current_sulco_id: Option<i32> = conn
        .query_row(
            "SELECT sulco_id FROM hoje_items WHERE id = ?1",
            [task_id],
            |row| row.get(0),
        )
        .ok()
        .flatten();
    let mut stmt = conn
        .prepare("SELECT id, text FROM sulco_items WHERE archived = 0 ORDER BY position ASC")
        .map_err(|e| e.to_string())?;
    let items = stmt
        .query_map([], |row| {
            let id: i32 = row.get(0)?;
            Ok(SulcoLinkOption {
                id,
                text: row.get(1)?,
                is_currently_linked: Some(id) == current_sulco_id,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(items)
}
