use crate::db::AppState;
use serde::Serialize;
use tauri::{command, State};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoriaItem {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub category: Option<String>,
    pub created_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResurfaceCard {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub created_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryFilter {
    pub name: String,
    pub count: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryOption {
    pub name: String,
    pub is_currently_selected: bool,
}

#[command]
pub fn get_memoria_items(
    state: State<AppState>,
    category: Option<String>,
) -> Result<Vec<MemoriaItem>, String> {
    let items = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        let (query, params_vec): (String, Vec<String>) = match &category {
            Some(cat) => (
                "SELECT id, title, text, category, created_at FROM memoria_items WHERE archived = 0 AND category = ?1 ORDER BY created_at DESC".to_string(),
                vec![cat.clone()],
            ),
            None => (
                "SELECT id, title, text, category, created_at FROM memoria_items WHERE archived = 0 ORDER BY created_at DESC".to_string(),
                vec![],
            ),
        };
        let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
        let x = stmt
            .query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
                Ok(MemoriaItem {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    text: row.get(2)?,
                    category: row.get(3)?,
                    created_at: row.get(4)?,
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
pub fn get_resurfaced_item(state: State<AppState>) -> Result<Option<ResurfaceCard>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let result = conn.query_row(
        "SELECT id, title, text, created_at FROM memoria_items
         WHERE archived = 0
         ORDER BY COALESCE(resurfaced_at, created_at) ASC
         LIMIT 1",
        [],
        |row| {
            Ok(ResurfaceCard {
                id: row.get(0)?,
                title: row.get(1)?,
                text: row.get(2)?,
                created_at: row.get(3)?,
            })
        },
    );
    match result {
        Ok(item) => Ok(Some(item)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub fn keep_memoria_item_active(state: State<AppState>, id: i32) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE memoria_items SET resurfaced_at = datetime('now') WHERE id = ?1",
        [id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn archive_memoria_item(state: State<AppState>, id: i32) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute("UPDATE memoria_items SET archived = 1 WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn get_categoria_filters(state: State<AppState>) -> Result<Vec<CategoryFilter>, String> {
    let items = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare(
                "SELECT category, COUNT(*) FROM memoria_items
                 WHERE archived = 0 AND category IS NOT NULL
                 GROUP BY category ORDER BY category ASC",
            )
            .map_err(|e| e.to_string())?;
        let x = stmt
            .query_map([], |row| {
                Ok(CategoryFilter {
                    name: row.get(0)?,
                    count: row.get(1)?,
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
pub fn get_categoria_options(
    state: State<AppState>,
    item_id: i32,
) -> Result<Vec<CategoryOption>, String> {
    let items = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        let current_category: Option<String> = conn
            .query_row(
                "SELECT category FROM memoria_items WHERE id = ?1",
                [item_id],
                |row| row.get(0),
            )
            .ok()
            .flatten();
        let mut stmt = conn
            .prepare("SELECT name FROM memoria_categories ORDER BY name ASC")
            .map_err(|e| e.to_string())?;
        let x = stmt
            .query_map([], |row| {
                let name: String = row.get(0)?;
                let is_currently_selected = Some(&name) == current_category.as_ref();
                Ok(CategoryOption {
                    name,
                    is_currently_selected,
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
pub fn set_memoria_item_category(
    state: State<AppState>,
    id: i32,
    category: Option<String>,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE memoria_items SET category = ?1 WHERE id = ?2",
        rusqlite::params![category, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn create_memoria_category(state: State<AppState>, name: String) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT OR IGNORE INTO memoria_categories (name) VALUES (?1)",
        [&name],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn update_memoria_text(state: State<AppState>, id: i32, text: String) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE memoria_items SET text = ?1 WHERE id = ?2",
        rusqlite::params![text, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn update_memoria_title(state: State<AppState>, id: i32, title: String) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE memoria_items SET title = ?1 WHERE id = ?2",
        rusqlite::params![title, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
