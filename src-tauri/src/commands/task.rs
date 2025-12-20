use crate::database::entities::{tasks, task_steps, task_execution_logs};
use crate::database::manager::DatabaseManager;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, QueryFilter, ColumnTrait, QueryOrder};
use tauri::{State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::core::orchestrator::TaskRunner;

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStepDTO {
    pub id: Option<i32>,
    pub action_type: String,
    pub params: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskDTO {
    pub id: String,
    pub name: String,
    pub cron: Option<String>,
    pub enabled: bool,
    pub steps: Vec<TaskStepDTO>,
}

#[tauri::command]
pub async fn create_task(task: TaskDTO, state: State<'_, DatabaseManager>) -> Result<(), String> {
    let db = &state.connection;
    let id = Uuid::new_v4().to_string();

    let active = tasks::ActiveModel {
        id: Set(id.clone()),
        name: Set(task.name),
        cron_expression: Set(task.cron),
        enabled: Set(Some(task.enabled)),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    active.insert(db).await.map_err(|e| e.to_string())?;

    for (index, step) in task.steps.iter().enumerate() {
        let active_step = task_steps::ActiveModel {
            task_id: Set(Some(id.clone())),
            step_order: Set(index as i32),
            action_type: Set(step.action_type.clone()),
            params: Set(Some(step.params.clone())),
            ..Default::default()
        };
        active_step.insert(db).await.map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn run_task_now(id: String, state: State<'_, DatabaseManager>) -> Result<(), String> {
    TaskRunner::run(id, &state).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn list_tasks(state: State<'_, DatabaseManager>) -> Result<Vec<tasks::Model>, String> {
    let db = &state.connection;
    let tasks = tasks::Entity::find().all(db).await.map_err(|e| e.to_string())?;
    Ok(tasks)
}

#[tauri::command]
pub async fn delete_task(id: String, state: State<'_, DatabaseManager>) -> Result<(), String> {
    let db = &state.connection;
    tasks::Entity::delete_by_id(id).exec(db).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_task_logs(task_id: String, state: State<'_, DatabaseManager>) -> Result<Vec<task_execution_logs::Model>, String> {
    let db = &state.connection;
    let logs = task_execution_logs::Entity::find()
        .filter(task_execution_logs::Column::TaskId.eq(task_id))
        .order_by_desc(task_execution_logs::Column::StartTime)
        .all(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(logs)
}
