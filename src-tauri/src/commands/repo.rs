use crate::database::entities::{repo_groups, repositories};
use crate::database::manager::DatabaseManager;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::State;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoNode {
    pub id: String,
    pub name: String,
    pub path: String,
    pub group_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupNode {
    pub id: String,
    pub name: String,
    pub children: Vec<GroupNode>,
    pub repos: Vec<RepoNode>,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn create_repo_group(
    name: String,
    parent_id: Option<String>,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    println!(
        "[create_repo_group] Received: name={}, parent_id={:?}",
        name, parent_id
    );

    let db = &state.connection;
    let id = Uuid::new_v4().to_string();

    let active = repo_groups::ActiveModel {
        id: Set(id.clone()),
        name: Set(name.clone()),
        parent_id: Set(parent_id.clone()),
        sort_order: Set(0),
    };

    println!(
        "[create_repo_group] Inserting group with id={}, name={}, parent_id={:?}",
        id, name, parent_id
    );
    active.insert(db).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_repo_group(
    id: String,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;
    // SeaORM Cascade Delete should handle children if configured in DB,
    // but code-first migration might rely on logical deletion or explicit checks.
    // For now, simple delete.
    repo_groups::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_repo_group(
    id: String,
    name: String,
    parent_id: Option<String>,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;
    let existing = repo_groups::Entity::find_by_id(&id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(model) = existing {
        let mut active: repo_groups::ActiveModel = model.into();
        active.name = Set(name);
        active.parent_id = Set(parent_id);
        active.update(db).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn add_repository(
    name: String,
    path: String,
    url: Option<String>,
    group_id: Option<String>,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    // Validate Git Path
    let repo_path = Path::new(&path);
    if !repo_path.exists() {
        return Err("Path does not exist".to_string());
    }

    // Check if it's a git repo
    match git2::Repository::open(repo_path) {
        Ok(_) => {}
        Err(_) => return Err("Invalid git repository".to_string()),
    }

    let db = &state.connection;
    let id = Uuid::new_v4().to_string();

    let active = repositories::ActiveModel {
        id: Set(id),
        name: Set(name),
        local_path: Set(path),
        remote_url: Set(url),
        group_id: Set(group_id),
        auth_type: Set("none".to_string()), // Default
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };

    active.insert(db).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_repository(
    id: String,
    name: String,
    path: String,
    url: Option<String>,
    group_id: Option<String>,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;
    let existing = repositories::Entity::find_by_id(&id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(model) = existing {
        let mut active: repositories::ActiveModel = model.into();
        active.name = Set(name);
        active.local_path = Set(path);
        active.remote_url = Set(url);
        active.group_id = Set(group_id);
        active.update(db).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_repository(
    id: String,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;
    repositories::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_repository_group(
    id: String,
    group_id: Option<String>,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;
    let existing = repositories::Entity::find_by_id(&id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(model) = existing {
        let mut active: repositories::ActiveModel = model.into();
        active.group_id = Set(group_id);
        active.update(db).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_repo_group_parent(
    id: String,
    parent_id: Option<String>,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;
    // Prevent setting parent to self
    if parent_id.as_ref() == Some(&id) {
        return Err("Cannot set group as its own parent".to_string());
    }

    let existing = repo_groups::Entity::find_by_id(&id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(model) = existing {
        let mut active: repo_groups::ActiveModel = model.into();
        active.parent_id = Set(parent_id);
        active.update(db).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn list_repo_tree(state: State<'_, DatabaseManager>) -> Result<Vec<GroupNode>, String> {
    let db = &state.connection;

    let all_groups = repo_groups::Entity::find()
        .order_by_asc(repo_groups::Column::SortOrder)
        .all(db)
        .await
        .map_err(|e| e.to_string())?;

    let all_repos = repositories::Entity::find()
        .all(db)
        .await
        .map_err(|e| e.to_string())?;

    // Build Tree (Recursive or Iterative)
    // Simplified approach: Return root nodes (parent_id is null) and recursively fill children

    // Group by parent_id
    // This logic is complex to implement purely in one pass without a HashMap or similar structure.
    // For simplicity in this iteration, we will just return a flat structure or
    // do a simple build. Let's do a recursive build function.

    // 1. Organize groups by parent_id
    // 2. Organize repos by group_id

    // NOTE: This implementation assumes small number of groups/repos.
    // Ideally we fetch everything and reconstruct in memory.

    Ok(build_tree(&all_groups, &all_repos, None))
}

fn build_tree(
    groups: &[repo_groups::Model],
    repos: &[repositories::Model],
    parent_id: Option<&String>,
) -> Vec<GroupNode> {
    let mut nodes = Vec::new();

    println!(
        "[build_tree] Looking for groups with parent_id: {:?}",
        parent_id
    );

    for group in groups {
        println!(
            "[build_tree] Checking group '{}' (id: {}) with parent_id: {:?}",
            group.name, group.id, group.parent_id
        );

        if group.parent_id.as_ref() == parent_id {
            println!("[build_tree] MATCHED! Adding '{}' to tree", group.name);
            let children = build_tree(groups, repos, Some(&group.id));
            let group_repos: Vec<RepoNode> = repos
                .iter()
                .filter(|r| r.group_id.as_ref() == Some(&group.id))
                .map(|r| RepoNode {
                    id: r.id.clone(),
                    name: r.name.clone(),
                    path: r.local_path.clone(),
                    group_id: r.group_id.clone(),
                })
                .collect();

            nodes.push(GroupNode {
                id: group.id.clone(),
                name: group.name.clone(),
                children,
                repos: group_repos,
            });
        }
    }

    // Add root repos (repos without group) - usually this might be a "Root" pseudo-group on frontend,
    // or we handle them separately. For `list_repo_tree` signature returning `Vec<GroupNode>`,
    // we might need a "Uncategorized" group if there are root repos, or change return type.
    // However, looking at the struct `GroupNode`, it represents a folder.
    // If the frontend expects a mixed list at root, the return type should likely be `Vec<TreeNodeEnum>`.
    // But per task spec: "provide a unified tree structure JSON".
    // Let's assume the root list contains top-level Groups. Top-level Repos might be missed here if we strictly follow `Vec<GroupNode>`.

    // FIX: Let's create a "virtual" root or change the return logic if needed.
    // Or, better, just map root repos into a "Uncategorized" group automatically if they exist,
    // OR allow the frontend to request root repos separately.

    // Current logic only returns Groups at the top level.
    // If parent_id is None (root call), we should perhaps return a special structure?
    // Let's stick to `Vec<GroupNode>` for now. Repos at root might need a dummy group wrapper
    // or we change `GroupNode` to be `TreeNode` which can be Group or Repo.

    // For root level (parent_id is None), also append repositories that have no group_id
    if parent_id.is_none() {
        // Find root repos
        let root_repos: Vec<RepoNode> = repos
            .iter()
            .filter(|r| r.group_id.is_none())
            .map(|r| RepoNode {
                id: r.id.clone(),
                name: r.name.clone(),
                path: r.local_path.clone(),
                group_id: None,
            })
            .collect();

        if !root_repos.is_empty() {
            // Create a virtual "Root" group or Uncategorized
            // But wait, the frontend RepoTree logic expects GroupNode to have children/repos.
            // If we want root repos to appear at the same level as root groups, the return type `Vec<GroupNode>` is restrictive.
            // However, we can hack it by adding a dummy group named "Uncategorized" (or similar)
            // OR we can change the frontend to handle a mix.
            // Given the signature `Result<Vec<GroupNode>, String>`, we must return GroupNodes.
            // Let's add a virtual group for root repositories.

            nodes.push(GroupNode {
                id: "root_virtual".to_string(),
                name: "Uncategorized".to_string(), // TODO: i18n?
                children: vec![],
                repos: root_repos,
            });
        }
    }

    nodes
}
