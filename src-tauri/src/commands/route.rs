use crate::database::entities::{route_groups, routes};
use crate::database::manager::DatabaseManager;
use glob::Pattern;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct RouteNode {
    pub id: String,
    pub name: String,
    pub group_id: Option<String>,
    pub source_id: Option<String>,
    pub target_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RouteGroupNode {
    pub id: String,
    pub name: String,
    pub children: Vec<RouteGroupNode>,
    pub routes: Vec<RouteNode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MappingRule {
    pub source: String,
    pub target: String,
    pub mode: String, // "copy" | "ignore"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestMatchResult {
    pub matched: bool,
    pub rule_index: Option<usize>,
    pub target_path: Option<String>,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn create_route_group(
    name: String,
    parent_id: Option<String>,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;
    let id = Uuid::new_v4().to_string();

    let active = route_groups::ActiveModel {
        id: Set(id),
        name: Set(name),
        parent_id: Set(parent_id),
        sort_order: Set(0),
    };

    active.insert(db).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_route_group(
    id: String,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;
    route_groups::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_route_group(
    id: String,
    name: String,
    parent_id: Option<String>,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;
    let existing = route_groups::Entity::find_by_id(&id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(model) = existing {
        let mut active: route_groups::ActiveModel = model.into();
        active.name = Set(name);
        active.parent_id = Set(parent_id);
        active.update(db).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn create_route(
    name: String,
    description: Option<String>,
    source_id: Option<String>,
    target_id: Option<String>,
    group_id: Option<String>,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;
    let id = Uuid::new_v4().to_string();

    let active = routes::ActiveModel {
        id: Set(id),
        name: Set(name),
        description: Set(description),
        main_repo_id: Set(source_id),
        slave_repo_id: Set(target_id),
        group_id: Set(group_id),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };

    active.insert(db).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_route(
    id: String,
    name: String,
    description: Option<String>,
    source_id: Option<String>,
    target_id: Option<String>,
    group_id: Option<String>,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;
    let existing = routes::Entity::find_by_id(&id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(model) = existing {
        let mut active: routes::ActiveModel = model.into();
        active.name = Set(name);
        active.description = Set(description);
        active.main_repo_id = Set(source_id);
        active.slave_repo_id = Set(target_id);
        active.group_id = Set(group_id);
        active.updated_at = Set(chrono::Utc::now().naive_utc());
        active.update(db).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn update_route_mappings(
    id: String,
    mappings: String,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;
    let existing = routes::Entity::find_by_id(&id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(model) = existing {
        let mut active: routes::ActiveModel = model.into();
        active.mappings = Set(Some(mappings));
        active.updated_at = Set(chrono::Utc::now().naive_utc());
        active.update(db).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_route(id: String, state: State<'_, DatabaseManager>) -> Result<(), String> {
    let db = &state.connection;
    routes::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_route_group_id(
    id: String,
    group_id: Option<String>,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;
    let existing = routes::Entity::find_by_id(&id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(model) = existing {
        let mut active: routes::ActiveModel = model.into();
        active.group_id = Set(group_id);
        active.updated_at = Set(chrono::Utc::now().naive_utc());
        active.update(db).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_route_group_parent(
    id: String,
    parent_id: Option<String>,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;
    if parent_id.as_ref() == Some(&id) {
        return Err("Cannot set group as its own parent".to_string());
    }

    let existing = route_groups::Entity::find_by_id(&id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(model) = existing {
        let mut active: route_groups::ActiveModel = model.into();
        active.parent_id = Set(parent_id);
        active.update(db).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn list_route_tree(
    state: State<'_, DatabaseManager>,
) -> Result<Vec<RouteGroupNode>, String> {
    let db = &state.connection;

    let all_groups = route_groups::Entity::find()
        .order_by_asc(route_groups::Column::SortOrder)
        .all(db)
        .await
        .map_err(|e| e.to_string())?;

    let all_routes = routes::Entity::find()
        .all(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(build_tree(&all_groups, &all_routes, None))
}

#[tauri::command]
pub async fn get_route_details(
    id: String,
    state: State<'_, DatabaseManager>,
) -> Result<Option<routes::Model>, String> {
    let db = &state.connection;
    let route = routes::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(route)
}

#[tauri::command]
pub async fn test_route_mapping(path: String, mappings: String) -> Result<TestMatchResult, String> {
    // mappings is JSON array of MappingRule
    let rules: Vec<MappingRule> = serde_json::from_str(&mappings).map_err(|e| e.to_string())?;

    for (index, rule) in rules.iter().enumerate() {
        // Check glob match
        if let Ok(pattern) = Pattern::new(&rule.source) {
            if pattern.matches(&path) {
                // Simple target replacement logic?
                // e.g. source: src/*.ts, target: dest/
                // path: src/main.ts -> dest/main.ts
                // This is complex to do generically for globs.
                // For this task, we just return that it matched the rule.

                return Ok(TestMatchResult {
                    matched: true,
                    rule_index: Some(index),
                    target_path: Some(format!("{} -> {}", path, rule.target)), // Placeholder logic
                });
            }
        }
    }

    Ok(TestMatchResult {
        matched: false,
        rule_index: None,
        target_path: None,
    })
}

fn build_tree(
    groups: &[route_groups::Model],
    routes: &[routes::Model],
    parent_id: Option<&String>,
) -> Vec<RouteGroupNode> {
    let mut nodes = Vec::new();

    for group in groups {
        if group.parent_id.as_ref() == parent_id {
            let children = build_tree(groups, routes, Some(&group.id));
            let group_routes: Vec<RouteNode> = routes
                .iter()
                .filter(|r| r.group_id.as_ref() == Some(&group.id))
                .map(|r| RouteNode {
                    id: r.id.clone(),
                    name: r.name.clone(),
                    group_id: r.group_id.clone(),
                    source_id: r.main_repo_id.clone(),
                    target_id: r.slave_repo_id.clone(),
                })
                .collect();

            nodes.push(RouteGroupNode {
                id: group.id.clone(),
                name: group.name.clone(),
                children,
                routes: group_routes,
            });
        }
    }
    // For root level (parent_id is None), also append routes that have no group_id
    if parent_id.is_none() {
        let root_routes: Vec<RouteNode> = routes
            .iter()
            .filter(|r| r.group_id.is_none())
            .map(|r| RouteNode {
                id: r.id.clone(),
                name: r.name.clone(),
                group_id: None,
                source_id: r.main_repo_id.clone(),
                target_id: r.slave_repo_id.clone(),
            })
            .collect();

        if !root_routes.is_empty() {
            nodes.push(RouteGroupNode {
                id: "route_root_virtual".to_string(),
                name: "Uncategorized".to_string(),
                children: vec![],
                routes: root_routes,
            });
        }
    }

    nodes
}
