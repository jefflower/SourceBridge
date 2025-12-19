// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

mod database;
mod commands;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::settings::get_setting,
            commands::settings::set_setting,
            commands::settings::get_all_settings,
            commands::repo::create_repo_group,
            commands::repo::delete_repo_group,
            commands::repo::update_repo_group,
            commands::repo::add_repository,
            commands::repo::update_repository,
            commands::repo::delete_repository,
            commands::repo::list_repo_tree
        ])
        .setup(|app| {
            let app_handle = app.handle();

            // Initialize Database
            tauri::async_runtime::block_on(async {
                let app_data_dir = app_handle.path().app_data_dir().expect("failed to get app data dir");
                let db_manager = database::manager::DatabaseManager::new(app_data_dir).await.expect("failed to initialize database");

                // Init defaults
                commands::settings::init_defaults(&db_manager).await.expect("failed to init settings");

                app_handle.manage(db_manager);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
