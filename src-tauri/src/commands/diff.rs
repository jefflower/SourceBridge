use crate::database::entities::{routes, repositories};
use crate::database::manager::DatabaseManager;
use crate::core::diff::{DiffEngine, DiffSummary};
use crate::commands::route::MappingRule;
use sea_orm::{EntityTrait};
use tauri::State;
use std::path::Path;

#[tauri::command]
pub async fn preview_route_diff(route_id: String, state: State<'_, DatabaseManager>) -> Result<DiffSummary, String> {
    let db = &state.connection;

    // 1. Get Route
    let route = routes::Entity::find_by_id(&route_id).one(db).await.map_err(|e| e.to_string())?
        .ok_or("Route not found")?;

    // 2. Get Source and Target Repos
    let source_repo = repositories::Entity::find_by_id(route.main_repo_id.clone().ok_or("No source repo")?).one(db).await.map_err(|e| e.to_string())?
        .ok_or("Source repo not found")?;
    let target_repo = repositories::Entity::find_by_id(route.slave_repo_id.clone().ok_or("No target repo")?).one(db).await.map_err(|e| e.to_string())?
        .ok_or("Target repo not found")?;

    // 3. Parse Mappings
    let mappings_json = route.mappings.ok_or("No mappings defined")?;
    let mappings: Vec<MappingRule> = serde_json::from_str(&mappings_json).map_err(|e| e.to_string())?;

    // 4. Scan
    let source_path = Path::new(&source_repo.local_path);
    let target_path = Path::new(&target_repo.local_path);

    DiffEngine::scan_changes_with_roots(source_path, target_path, &mappings).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_file_diff(source_path: Option<String>, target_path: Option<String>) -> Result<(String, String), String> {
    // Return (Original, Modified) content for Monaco
    DiffEngine::get_file_content_pair(source_path, target_path).map_err(|e| e.to_string())
}
