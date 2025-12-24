use crate::database::entities::workspace_config;
use crate::database::manager::DatabaseManager;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use tauri::State;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_workspace_config(
    group_id: String,
    state: State<'_, DatabaseManager>,
) -> Result<Option<workspace_config::Model>, String> {
    let db = &state.connection;
    let config = workspace_config::Entity::find_by_id(&group_id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn save_workspace_config(
    group_id: String,
    source_path: Option<String>,
    target_path: Option<String>,
    open_command: Option<String>,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;

    // Check if exists
    let existing = workspace_config::Entity::find_by_id(&group_id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(_) = existing {
        // Update
        let active = workspace_config::ActiveModel {
            group_id: Set(group_id),
            source_path: Set(source_path),
            target_path: Set(target_path),
            open_command: Set(open_command),
        };
        active.update(db).await.map_err(|e| e.to_string())?;
    } else {
        // Insert
        let active = workspace_config::ActiveModel {
            group_id: Set(group_id),
            source_path: Set(source_path),
            target_path: Set(target_path),
            open_command: Set(open_command),
        };
        active.insert(db).await.map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn launch_workspace(
    group_id: String,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;
    let config = workspace_config::Entity::find_by_id(&group_id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(cfg) = config {
        // Open paths
        if let Some(ref path) = cfg.source_path {
            if !path.is_empty() {
                let _ = crate::commands::git_ops::open_in_folder(path.clone());
            }
        }
        if let Some(ref path) = cfg.target_path {
             if !path.is_empty() {
                let _ = crate::commands::git_ops::open_in_folder(path.clone());
            }
        }

        // Run command
        if let Some(cmd_str) = cfg.open_command {
             if !cmd_str.is_empty() {
                // This is a bit tricky. "code ." usually needs to run in a shell.
                // Or maybe the user provides the full path to executable?
                // For safety and simplicity, we might interpret specific keywords or run as shell command.

                // If the user entered "code", we assume they want to open the source/target path in code?
                // Usually "Launch Workspace" implies opening the project in IDE.

                // Let's assume the command is run in the source path if it exists, else target path.
                let working_dir = cfg.source_path.clone().or(cfg.target_path.clone());

                if let Some(cwd) = working_dir {
                    #[cfg(target_os = "windows")]
                    std::process::Command::new("cmd")
                        .args(&["/c", &cmd_str])
                        .current_dir(cwd)
                        .spawn()
                        .map_err(|e| format!("Failed to launch command: {}", e))?;

                     #[cfg(not(target_os = "windows"))]
                     std::process::Command::new("sh")
                        .arg("-c")
                        .arg(&cmd_str)
                        .current_dir(cwd)
                        .spawn()
                        .map_err(|e| format!("Failed to launch command: {}", e))?;
                }
             }
        }
    } else {
        return Err("No workspace configuration found for this group".to_string());
    }

    Ok(())
}
