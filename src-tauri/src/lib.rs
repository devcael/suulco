pub mod db;
pub mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            db::init(app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // sulco
            services::sulco::get_active_sulco_items,
            services::sulco::get_sulco_detail,
            services::sulco::update_sulco_text,
            services::sulco::update_sulco_definition,
            services::sulco::archive_sulco_item,
            services::sulco::get_archived_sulco_items,
            services::sulco::restore_sulco_item,
            services::sulco::get_sulco_link_options,
            // hoje
            services::hoje::get_hoje_items,
            services::hoje::get_hoje_summary,
            services::hoje::toggle_task_status,
            services::hoje::delete_task,
            services::hoje::link_task_to_sulco,
            services::hoje::update_task_text,
            services::hoje::defer_task_to_inbox,
            services::hoje::flush_overdue_tasks,
            services::hoje::create_task_linked_to_sulco,
            // inbox
            services::inbox::get_all_tasks,
            services::inbox::get_tasks_without_sulco,
            services::inbox::get_tasks_by_sulco_id,
            services::inbox::get_inbox_sulco_filters,
            services::inbox::move_task_to_hoje,
            // memoria
            services::memoria::get_memoria_items,
            services::memoria::get_resurfaced_item,
            services::memoria::keep_memoria_item_active,
            services::memoria::archive_memoria_item,
            services::memoria::get_categoria_filters,
            services::memoria::get_categoria_options,
            services::memoria::set_memoria_item_category,
            services::memoria::create_memoria_category,
            services::memoria::update_memoria_text,
            services::memoria::update_memoria_title,
            // input
            services::input::create_global_item,
            // settings
            services::settings::get_db_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
