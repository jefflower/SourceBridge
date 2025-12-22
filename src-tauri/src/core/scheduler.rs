use crate::core::orchestrator::TaskRunner;
use crate::database::entities::tasks;
use crate::database::manager::DatabaseManager;
use chrono::TimeZone;
use cron::Schedule;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

pub struct SchedulerManager {
    db_manager: Arc<DatabaseManager>,
    running: Arc<RwLock<bool>>,
}

impl SchedulerManager {
    pub fn new(db_manager: Arc<DatabaseManager>) -> Self {
        Self {
            db_manager,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Start the scheduler background loop
    pub async fn start(&self) {
        let mut running = self.running.write().await;
        if *running {
            println!("[Scheduler] Already running");
            return;
        }
        *running = true;
        drop(running);

        println!("[Scheduler] Starting scheduler...");

        let db_manager = self.db_manager.clone();
        let running_flag = self.running.clone();

        // Spawn background task
        tokio::spawn(async move {
            println!("[Scheduler] Waiting 10 seconds before first check...");
            sleep(Duration::from_secs(10)).await;

            loop {
                // Check if we should stop
                let is_running = *running_flag.read().await;
                if !is_running {
                    println!("[Scheduler] Stopping scheduler");
                    break;
                }

                // Check and execute due tasks
                if let Err(e) = check_and_run_due_tasks(&db_manager).await {
                    eprintln!("[Scheduler] Error checking tasks: {}", e);
                }

                // Sleep for 60 seconds before next check
                sleep(Duration::from_secs(60)).await;
            }
        });
    }

    #[allow(dead_code)]
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
        println!("[Scheduler] Stop requested");
    }
}

/// Check all enabled tasks and run those that are due according to their cron expression
async fn check_and_run_due_tasks(
    db_manager: &DatabaseManager,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = &db_manager.connection;

    // Get all enabled tasks with cron expressions
    let enabled_tasks = tasks::Entity::find()
        .filter(tasks::Column::Enabled.eq(Some(true)))
        .all(db)
        .await?;

    let now = chrono::Local::now();
    println!(
        "[Scheduler] Checking {} enabled tasks at {}",
        enabled_tasks.len(),
        now.format("%Y-%m-%d %H:%M:%S")
    );

    for task in enabled_tasks {
        if let Some(ref cron_expr) = task.cron_expression {
            if cron_expr.is_empty() {
                continue;
            }

            // Convert 5-field cron to 6-field (add seconds prefix "0 ")
            // Standard cron: min hour day month weekday
            // Cron crate:   sec min hour day month weekday
            let cron_6field = if cron_expr.split_whitespace().count() == 5 {
                format!("0 {}", cron_expr)
            } else {
                cron_expr.clone()
            };

            // Parse cron expression
            let schedule = match Schedule::from_str(&cron_6field) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!(
                        "[Scheduler] Invalid cron for task {}: {} -> {} - {}",
                        task.id, cron_expr, cron_6field, e
                    );
                    continue;
                }
            };

            // Get next scheduled time
            let next_run = schedule.upcoming(chrono::Local).take(1).next();

            // Check if this task should run now (within the last minute)
            let should_run = next_run
                .map(|next| {
                    let diff = next.signed_duration_since(now);
                    // Run if next execution is within 60 seconds
                    diff.num_seconds() >= 0 && diff.num_seconds() < 60
                })
                .unwrap_or(false);

            // Log task check details
            if let Some(next) = next_run {
                println!(
                    "[Scheduler] Task '{}' next run: {}, should_run: {}",
                    task.name,
                    next.format("%Y-%m-%d %H:%M:%S"),
                    should_run
                );
            }

            // Also check if we missed the last run (for tasks that haven't run recently)
            let missed_run = if let Some(last_run) = task.last_run_time {
                let last_run_local = chrono::Local
                    .from_local_datetime(&last_run)
                    .single()
                    .unwrap_or_else(chrono::Local::now);
                schedule
                    .after(&last_run_local)
                    .take(1)
                    .next()
                    .map(|next_after_last| next_after_last < now)
                    .unwrap_or(false)
            } else {
                // Never run before, check if it should have run already
                schedule
                    .upcoming(chrono::Local)
                    .take(1)
                    .next()
                    .map(|next| {
                        let diff = now.signed_duration_since(next);
                        diff.num_seconds() >= 0 && diff.num_seconds() < 120
                    })
                    .unwrap_or(false)
            };

            if should_run || missed_run {
                println!(
                    "[Scheduler] Running scheduled task: {} ({})",
                    task.name, task.id
                );

                // Run task in background
                let task_id = task.id.clone();
                let db_manager_clone = db_manager.clone();

                tokio::spawn(async move {
                    if let Err(e) = TaskRunner::run(task_id.clone(), &db_manager_clone).await {
                        eprintln!("[Scheduler] Task {} failed: {}", task_id, e);
                    }
                });
            }
        }
    }

    Ok(())
}
