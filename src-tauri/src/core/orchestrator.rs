use anyhow::Result;
use std::process::Command;
use serde::{Deserialize, Serialize};
use crate::database::entities::{tasks, task_steps, task_execution_logs};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, QueryOrder, DatabaseConnection};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub enum StepType {
    Script,
    Git,
    Sync
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptParams {
    pub script: String,
    pub continue_on_error: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitParams {
    pub repo_id: String,
    pub operation: String, // "pull", "push", "checkout"
    pub arg: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncParams {
    pub route_id: String,
}

pub trait StepExecutor {
    fn execute(&self, params: &str) -> Result<String>;
}

pub struct ScriptExecutor;
impl StepExecutor for ScriptExecutor {
    fn execute(&self, params: &str) -> Result<String> {
        let p: ScriptParams = serde_json::from_str(params)?;

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", &p.script])
                .output()?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&p.script)
                .output()?
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() && !p.continue_on_error {
            return Err(anyhow::anyhow!("Script failed: {}", stderr));
        }

        Ok(format!("{}\n{}", stdout, stderr))
    }
}

// Placeholder for Git and Sync executors
pub struct GitExecutor;
impl StepExecutor for GitExecutor {
    fn execute(&self, _params: &str) -> Result<String> {
        // TODO: Implement Git logic using git2 or command
        Ok("Git operation executed (Mock)".to_string())
    }
}

pub struct SyncExecutor;
impl StepExecutor for SyncExecutor {
    fn execute(&self, _params: &str) -> Result<String> {
        // TODO: Implement Sync logic using Route module
        Ok("Sync operation executed (Mock)".to_string())
    }
}

pub struct TaskRunner;

impl TaskRunner {
    pub async fn run(task_id: String, db: &DatabaseConnection) -> Result<()> {
        // 1. Fetch Task and Steps
        let task = tasks::Entity::find_by_id(&task_id).one(db).await?
            .ok_or(anyhow::anyhow!("Task not found"))?;

        let steps = task_steps::Entity::find()
            .filter(task_steps::Column::TaskId.eq(&task_id))
            .order_by_asc(task_steps::Column::StepOrder)
            .all(db)
            .await?;

        // 2. Create Log Entry
        let active_log = task_execution_logs::ActiveModel {
            task_id: Set(Some(task_id.clone())),
            start_time: Set(Some(Utc::now().naive_utc())),
            status: Set(Some("running".to_string())),
            output_log: Set(Some(String::new())),
            ..Default::default()
        };
        let log_entry = active_log.insert(db).await?;
        let mut full_log = String::new();

        // 3. Execute Steps
        let mut success = true;
        for step in steps {
            let executor: Box<dyn StepExecutor> = match step.action_type.as_str() {
                "script" => Box::new(ScriptExecutor),
                "git" => Box::new(GitExecutor),
                "sync" => Box::new(SyncExecutor),
                _ => {
                    full_log.push_str(&format!("Unknown step type: {}\n", step.action_type));
                    success = false;
                    break;
                }
            };

            let params = step.params.unwrap_or_default();
            match executor.execute(&params) {
                Ok(output) => {
                    full_log.push_str(&format!("Step {} Success:\n{}\n", step.step_order, output));
                },
                Err(e) => {
                    full_log.push_str(&format!("Step {} Failed:\n{}\n", step.step_order, e));
                    success = false;
                    break;
                }
            }
        }

        // 4. Update Log and Task Status
        let status = if success { "success" } else { "failed" };

        let mut active_log: task_execution_logs::ActiveModel = log_entry.into();
        active_log.end_time = Set(Some(Utc::now().naive_utc()));
        active_log.status = Set(Some(status.to_string()));
        active_log.output_log = Set(Some(full_log));
        active_log.update(db).await?;

        let mut active_task: tasks::ActiveModel = task.into();
        active_task.last_run_status = Set(Some(status.to_string()));
        active_task.last_run_time = Set(Some(Utc::now().naive_utc()));
        active_task.update(db).await?;

        Ok(())
    }
}
