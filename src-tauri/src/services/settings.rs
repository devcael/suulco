use crate::db::AppState;
use tauri::{command, State};

#[command]
pub fn get_db_path(state: State<AppState>) -> String {
    state.db_path.to_string_lossy().to_string()
}
