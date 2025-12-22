use crate::database::entities::settings;
use crate::database::manager::DatabaseManager;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use std::collections::HashMap;
use tauri::State;

#[tauri::command]
pub async fn get_setting(
    key: String,
    state: State<'_, DatabaseManager>,
) -> Result<Option<String>, String> {
    let db = &state.connection;
    let setting = settings::Entity::find_by_id(&key)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(setting.map(|s| s.value))
}

#[tauri::command]
pub async fn set_setting(
    key: String,
    value: String,
    state: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let db = &state.connection;

    // Check if exists
    let existing = settings::Entity::find_by_id(&key)
        .one(db)
        .await
        .map_err(|e| e.to_string())?;

    match existing {
        Some(m) => {
            let mut active: settings::ActiveModel = m.into();
            active.value = Set(value);
            active.update(db).await.map_err(|e| e.to_string())?;
        }
        None => {
            let active = settings::ActiveModel {
                key: Set(key),
                value: Set(value),
            };
            active.insert(db).await.map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn get_all_settings(
    state: State<'_, DatabaseManager>,
) -> Result<HashMap<String, String>, String> {
    let db = &state.connection;
    let settings = settings::Entity::find()
        .all(db)
        .await
        .map_err(|e| e.to_string())?;

    let mut map = HashMap::new();
    for s in settings {
        map.insert(s.key, s.value);
    }
    Ok(map)
}

pub async fn init_defaults(state: &DatabaseManager) -> Result<(), String> {
    let defaults = vec![
        ("theme", "system"),
        ("language", "en"), // Standardized to 'en'
        ("git_path", "git"),
    ];

    let db = &state.connection;

    for (key, value) in defaults {
        let exists = settings::Entity::find_by_id(key)
            .one(db)
            .await
            .map_err(|e| e.to_string())?
            .is_some();

        if !exists {
            let active = settings::ActiveModel {
                key: Set(key.to_string()),
                value: Set(value.to_string()),
            };
            active.insert(db).await.map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
