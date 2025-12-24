use crate::database::entities::{task_execution_logs, task_steps, tasks};
use anyhow::Result;
use chrono::Local;
use once_cell::sync::Lazy;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::process::Command;
use std::sync::Mutex;

static RUNNING_TASKS: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

#[derive(Serialize, Deserialize, Debug)]
pub enum StepType {
    Script,
    Git,
    Sync,
    #[serde(rename = "AI_PROMPT")]
    AiPrompt,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptParams {
    pub script: String,
    pub continue_on_error: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AIParams {
    pub system_prompt: Option<String>,
    pub user_prompt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitParams {
    pub repo_id: String,
    pub operation: String, // "pull", "push", "fetch", "reset"
    pub arg: Option<String>,
    #[serde(default)]
    pub force_push: bool, // Only used for push operation
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
            Command::new("cmd").args(["/C", &p.script]).output()?
        } else {
            Command::new("sh").arg("-c").arg(&p.script).output()?
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

use crate::core::ai_service::AIService;
use crate::core::sync::SyncEngine;
use crate::database::entities::repositories;
use crate::database::manager::DatabaseManager;

/// Helper to execute a single shell command in a directory
fn run_command(args: Vec<&str>, dir: &str) -> Result<(String, String, bool)> {
    let output = Command::new("git").args(&args).current_dir(dir).output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    Ok((stdout, stderr, output.status.success()))
}

/// Reset the local repository to match its remote counterpart exactly
async fn execute_reset_to_remote(repo_name: &str, repo_path: &str) -> Result<String> {
    let mut logs = format!(
        "[Git RESET_REMOTE] Repository: {} ({})\n",
        repo_name, repo_path
    );

    // 1. Fetch latest from remote
    logs.push_str("> git fetch --all\n");
    let (stdout, stderr, success) = run_command(vec!["fetch", "--all"], repo_path)?;
    logs.push_str(&stdout);
    logs.push_str(&stderr);
    if !success {
        return Err(anyhow::anyhow!("git fetch failed:\n{}", logs));
    }

    // 2. Get current branch name
    let (stdout, _, success) = run_command(vec!["rev-parse", "--abbrev-ref", "HEAD"], repo_path)?;
    if !success {
        return Err(anyhow::anyhow!("Failed to get current branch name"));
    }
    let branch = stdout.trim();
    if branch.is_empty() || branch == "HEAD" {
        return Err(anyhow::anyhow!("Repository is in detached HEAD state"));
    }

    // 3. Reset to remote counterpart (assuming origin/branch)
    let remote_target = format!("origin/{}", branch);
    logs.push_str(&format!("> git reset --hard {}\n", remote_target));
    let (stdout, stderr, success) =
        run_command(vec!["reset", "--hard", &remote_target], repo_path)?;
    logs.push_str(&stdout);
    logs.push_str(&stderr);

    if !success {
        return Err(anyhow::anyhow!("git reset --hard failed:\n{}", logs));
    }

    Ok(logs)
}

/// Execute a real Git operation (pull, push, fetch, reset) on the specified repository
async fn execute_git_operation(params: &GitParams, db_manager: &DatabaseManager) -> Result<String> {
    let db = &db_manager.connection;

    // 1. Get repository info from database
    let repo = repositories::Entity::find_by_id(&params.repo_id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Repository not found: {}", params.repo_id))?;

    let repo_path = &repo.local_path;

    // 2. Handle special operations that require multiple commands
    if params.operation == "reset_remote" {
        return execute_reset_to_remote(&repo.name, repo_path).await;
    }

    // 3. Build command based on operation
    let (cmd, git_args): (&str, Vec<&str>) = match params.operation.as_str() {
        "pull" => ("gemini", vec!["/pull", "-y"]),
        "push" => ("gemini", vec!["/commit", "-y"]),
        "fetch" => ("git", vec!["fetch", "--all"]),
        "reset" => ("git", vec!["reset", "--hard", "HEAD"]),
        _ => {
            return Err(anyhow::anyhow!(
                "Unknown git operation: {}",
                params.operation
            ))
        }
    };

    println!(
        "[execute_git_operation] Running: {} {:?} in {}",
        cmd, git_args, repo_path
    );

    // 4. Execute command
    let output = Command::new(cmd)
        .args(&git_args)
        .current_dir(repo_path)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let result = format!(
        "[Git {}] Repository: {} ({})\n{}{}",
        params.operation.to_uppercase(),
        repo.name,
        repo_path,
        stdout,
        stderr
    );

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Git {} failed:\n{}",
            params.operation,
            result
        ));
    }

    println!("[execute_git_operation] Success: {}", result.trim());
    Ok(result)
}

#[allow(dead_code)]
pub struct SyncExecutor<'a> {
    pub db_manager: &'a DatabaseManager,
}

// We need async trait or execute async.
// Since StepExecutor trait is synchronous `execute`, we can't await inside easily without block_on.
// Ideally refactor StepExecutor to async trait.
// For now, let's keep it simple and maybe refactor TaskRunner to handle async steps directly match.

pub struct TaskRunner;

impl TaskRunner {
    pub async fn run(task_id: String, db_manager: &DatabaseManager) -> Result<()> {
        // 0. Check if task is already running (Singleton check)
        {
            let mut running = RUNNING_TASKS.lock().unwrap();
            if running.contains(&task_id) {
                println!("[TaskRunner] Task {} is already running, skipping", task_id);
                return Ok(());
            }
            running.insert(task_id.clone());
        }

        // Ensure we remove it from the running set when we finish
        let result = Self::execute_task(task_id.clone(), db_manager).await;

        {
            let mut running = RUNNING_TASKS.lock().unwrap();
            running.remove(&task_id);
        }

        result
    }

    async fn execute_task(task_id: String, db_manager: &DatabaseManager) -> Result<()> {
        let db = &db_manager.connection;
        // 1. Fetch Task and Steps
        let task = tasks::Entity::find_by_id(&task_id)
            .one(db)
            .await?
            .ok_or(anyhow::anyhow!("Task not found"))?;

        let steps = task_steps::Entity::find()
            .filter(task_steps::Column::TaskId.eq(&task_id))
            .order_by_asc(task_steps::Column::StepOrder)
            .all(db)
            .await?;

        // 2. Create Log Entry
        let active_log = task_execution_logs::ActiveModel {
            task_id: Set(Some(task_id.clone())),
            start_time: Set(Some(Local::now().naive_local())),
            status: Set(Some("running".to_string())),
            output_log: Set(Some(String::new())),
            ..Default::default()
        };
        let log_entry = active_log.insert(db).await?;
        let mut full_log = String::new();

        // 3. Execute Steps
        let mut success = true;
        for step in steps {
            let params = step.params.unwrap_or_default();
            let result = match step.action_type.as_str() {
                "script" => ScriptExecutor.execute(&params),
                "git" => {
                    // Git needs async execution to fetch repo info from DB
                    let p: GitParams = serde_json::from_str(&params)?;
                    match execute_git_operation(&p, db_manager).await {
                        Ok(output) => Ok(output),
                        Err(e) => Err(e),
                    }
                }
                "sync" => {
                    // Sync needs async execution and DB access
                    let p: SyncParams = serde_json::from_str(&params)?;
                    match SyncEngine::execute_sync(&p.route_id, db_manager).await {
                        Ok(res) => Ok(res.logs),
                        Err(e) => Err(e),
                    }
                }
                "AI_PROMPT" => {
                    let p: AIParams = serde_json::from_str(&params)?;
                    match AIService::chat_completion(db_manager, p.system_prompt, p.user_prompt)
                        .await
                    {
                        Ok(res) => Ok(format!("[AI Response]\n{}", res)),
                        Err(e) => Err(e),
                    }
                }
                _ => Err(anyhow::anyhow!("Unknown step type: {}", step.action_type)),
            };

            match result {
                Ok(output) => {
                    full_log.push_str(&format!("Step {} Success:\n{}\n", step.step_order, output));
                }
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
        active_log.end_time = Set(Some(Local::now().naive_local()));
        active_log.status = Set(Some(status.to_string()));
        active_log.output_log = Set(Some(full_log));
        active_log.update(db).await?;

        let mut active_task: tasks::ActiveModel = task.into();
        active_task.last_run_status = Set(Some(status.to_string()));
        active_task.last_run_time = Set(Some(Local::now().naive_local()));
        active_task.update(db).await?;

        Ok(())
    }
}
