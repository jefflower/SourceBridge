use crate::core::orchestrator::TaskRunner;
use crate::database::entities::{task_execution_logs, task_steps, tasks};
use crate::database::manager::DatabaseManager;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStepDTO {
    pub id: Option<i32>,
    pub action_type: String,
    pub params: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskDTO {
    pub id: Option<String>,
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
pub async fn update_task(task: TaskDTO, state: State<'_, DatabaseManager>) -> Result<(), String> {
    println!("[update_task] Received task: {:?}", task);
    let db = &state.connection;
    let task_id = task.id.ok_or("Task ID is required for update")?;
    println!("[update_task] Task ID: {}", task_id);

    // 1. Update task basic info
    let existing = tasks::Entity::find_by_id(&task_id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Task not found")?;

    let mut active: tasks::ActiveModel = existing.into();
    active.name = Set(task.name);
    active.cron_expression = Set(task.cron);
    active.enabled = Set(Some(task.enabled));
    active.update(db).await.map_err(|e| e.to_string())?;
    println!("[update_task] Updated task basic info");

    // 2. Delete all existing steps
    let delete_result = task_steps::Entity::delete_many()
        .filter(task_steps::Column::TaskId.eq(&task_id))
        .exec(db)
        .await
        .map_err(|e| e.to_string())?;
    println!(
        "[update_task] Deleted {} existing steps",
        delete_result.rows_affected
    );

    // 3. Insert new steps
    println!("[update_task] Inserting {} new steps", task.steps.len());
    for (index, step) in task.steps.iter().enumerate() {
        println!(
            "[update_task] Step {}: action_type={}, params={}",
            index, step.action_type, step.params
        );
        let active_step = task_steps::ActiveModel {
            task_id: Set(Some(task_id.clone())),
            step_order: Set(index as i32),
            action_type: Set(step.action_type.clone()),
            params: Set(Some(step.params.clone())),
            ..Default::default()
        };
        active_step.insert(db).await.map_err(|e| e.to_string())?;
    }
    println!("[update_task] Successfully updated task");

    Ok(())
}

#[tauri::command]
pub async fn run_task_now(id: String, state: State<'_, DatabaseManager>) -> Result<(), String> {
    TaskRunner::run(id, &state)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn list_tasks(state: State<'_, DatabaseManager>) -> Result<Vec<tasks::Model>, String> {
    let db = &state.connection;
    let tasks = tasks::Entity::find()
        .all(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(tasks)
}

#[derive(Serialize)]
pub struct TaskWithSteps {
    #[serde(flatten)]
    pub task: tasks::Model,
    pub steps: Vec<task_steps::Model>,
}

#[tauri::command]
pub async fn get_task_with_steps(
    id: String,
    state: State<'_, DatabaseManager>,
) -> Result<TaskWithSteps, String> {
    let db = &state.connection;

    let task = tasks::Entity::find_by_id(&id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Task not found")?;

    let steps = task_steps::Entity::find()
        .filter(task_steps::Column::TaskId.eq(&id))
        .order_by_asc(task_steps::Column::StepOrder)
        .all(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(TaskWithSteps { task, steps })
}

#[tauri::command]
pub async fn delete_task(id: String, state: State<'_, DatabaseManager>) -> Result<(), String> {
    let db = &state.connection;
    tasks::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_task_logs(
    task_id: String,
    state: State<'_, DatabaseManager>,
) -> Result<Vec<task_execution_logs::Model>, String> {
    let db = &state.connection;
    let logs = task_execution_logs::Entity::find()
        .filter(task_execution_logs::Column::TaskId.eq(task_id))
        .order_by_desc(task_execution_logs::Column::StartTime)
        .all(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(logs)
}
