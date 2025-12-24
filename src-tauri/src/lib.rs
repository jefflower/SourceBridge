// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::Manager;

mod commands;
mod core;
mod database;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
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
            commands::repo::update_repository_group,
            commands::repo::update_repo_group_parent,
            commands::repo::list_repo_tree,
            commands::route::create_route_group,
            commands::route::delete_route_group,
            commands::route::update_route_group,
            commands::route::create_route,
            commands::route::update_route,
            commands::route::update_route_mappings,
            commands::route::delete_route,
            commands::route::update_route_group_id,
            commands::route::update_route_group_parent,
            commands::route::list_route_tree,
            commands::route::get_route_details,
            commands::route::test_route_mapping,
            commands::route::preview_glob_matches,
            commands::route::sync_route,
            commands::task::create_task,
            commands::task::update_task,
            commands::task::run_task_now,
            commands::task::list_tasks,
            commands::task::get_task_with_steps,
            commands::task::delete_task,
            commands::task::get_task_logs,
            commands::diff::preview_route_diff,
            commands::diff::get_file_diff,
            commands::repo_scan::scan_local_repos,
            commands::repo_scan::import_scanned_repos,
            commands::git_ops::get_git_branches,
            commands::git_ops::switch_git_branch,
            commands::git_ops::get_git_log,
            commands::git_ops::open_in_folder,
            commands::git_ops::open_in_terminal,
            commands::git_ops::open_in_ide,
            commands::terminal::run_shell_command,
            commands::workspace::get_workspace_config,
            commands::workspace::save_workspace_config,
            commands::workspace::launch_workspace,
            commands::status::get_repos_status,
            commands::status::scan_dependencies,
            commands::ai::generate_ai_response,
            commands::ai::generate_release_notes,
            commands::ai::explain_diff,
            commands::report::generate_weekly_report
        ])
        .setup(|app| {
            let app_handle = app.handle();

            // System Tray
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::with_id("tray")
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            // Initialize Database and Scheduler
            tauri::async_runtime::block_on(async {
                let app_data_dir = app_handle
                    .path()
                    .app_data_dir()
                    .expect("failed to get app data dir");
                let db_manager = database::manager::DatabaseManager::new(app_data_dir)
                    .await
                    .expect("failed to initialize database");

                // Init defaults
                commands::settings::init_defaults(&db_manager)
                    .await
                    .expect("failed to init settings");

                // Start scheduler
                let db_manager_arc = std::sync::Arc::new(db_manager.clone());
                let scheduler = core::scheduler::SchedulerManager::new(db_manager_arc);
                scheduler.start().await;

                // Keep scheduler alive by managing it (optional: store in state if needed)
                // For now, it runs as a background task

                app_handle.manage(db_manager);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
