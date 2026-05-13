use crate::db::AppState;
use serde::Serialize;
use tauri::{command, State};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxItem {
    pub id: i32,
    pub text: String,
    pub is_done: bool,
    pub created_at: String,
    pub was_carried: bool,
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
pub struct InboxSulcoFilter {
    pub id: i32,
    pub text: String,
    pub task_count: i32,
}

fn row_to_inbox_item(row: &rusqlite::Row) -> rusqlite::Result<InboxItem> {
    let sulco_id: Option<i32> = row.get(5)?;
    let sulco_text: Option<String> = row.get(6)?;
    Ok(InboxItem {
        id: row.get(0)?,
        text: row.get(1)?,
        is_done: row.get::<_, i32>(2)? != 0,
        created_at: row.get(3)?,
        was_carried: row.get::<_, i32>(4)? != 0,
        linked_sulco: sulco_id
            .zip(sulco_text)
            .map(|(id, text)| SulcoRef { id, text }),
    })
}

const INBOX_SELECT: &str = "
    SELECT h.id, h.text, h.done, h.created_at, h.carried, s.id, s.text
    FROM hoje_items h
    LEFT JOIN sulco_items s ON h.sulco_id = s.id";

#[command]
pub fn get_all_tasks(state: State<AppState>) -> Result<Vec<InboxItem>, String> {
    let items = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        let query = format!("{} WHERE h.date IS NULL ORDER BY h.created_at DESC", INBOX_SELECT);
        let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
        let x = stmt
            .query_map([], row_to_inbox_item)
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string());
        x
    }?;
    Ok(items)
}

#[command]
pub fn get_tasks_without_sulco(state: State<AppState>) -> Result<Vec<InboxItem>, String> {
    let items = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        let query = format!(
            "{} WHERE h.date IS NULL AND h.sulco_id IS NULL ORDER BY h.created_at DESC",
            INBOX_SELECT
        );
        let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
        let x = stmt
            .query_map([], row_to_inbox_item)
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string());
        x
    }?;
    Ok(items)
}

#[command]
pub fn get_tasks_by_sulco_id(
    state: State<AppState>,
    sulco_id: i32,
) -> Result<Vec<InboxItem>, String> {
    let items = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        let query = format!(
            "{} WHERE h.date IS NULL AND h.sulco_id = ?1 ORDER BY h.created_at DESC",
            INBOX_SELECT
        );
        let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
        let x = stmt
            .query_map([sulco_id], row_to_inbox_item)
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string());
        x
    }?;
    Ok(items)
}

#[command]
pub fn get_inbox_sulco_filters(state: State<AppState>) -> Result<Vec<InboxSulcoFilter>, String> {
    let items = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare(
                "SELECT s.id, s.text, COUNT(h.id)
                 FROM sulco_items s
                 LEFT JOIN hoje_items h ON h.sulco_id = s.id AND h.date IS NULL
                 WHERE s.archived = 0
                 GROUP BY s.id
                 ORDER BY s.position ASC",
            )
            .map_err(|e| e.to_string())?;
        let x = stmt
            .query_map([], |row| {
                Ok(InboxSulcoFilter {
                    id: row.get(0)?,
                    text: row.get(1)?,
                    task_count: row.get(2)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string());
        x
    }?;
    Ok(items)
}

#[command]
pub fn move_task_to_hoje(state: State<AppState>, id: i32) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE hoje_items SET date = date('now') WHERE id = ?1",
        [id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
