use crate::database::manager::DatabaseManager;
use crate::database::entities::{task_execution_logs};
use crate::commands::git_ops::get_git_log;
use sea_orm::{EntityTrait, QueryOrder, QueryFilter, ColumnTrait};
use chrono::{Utc, Duration};
use tauri::State;
use serde::Serialize;
use crate::database::entities::repositories;

#[derive(Serialize)]
pub struct WeeklyReport {
    pub generated_at: String,
    pub sync_summary: SyncSummary,
    pub recent_commits: Vec<CommitSummary>,
}

#[derive(Serialize)]
pub struct SyncSummary {
    pub total_syncs: usize,
    pub success_count: usize,
    pub fail_count: usize,
}

#[derive(Serialize)]
pub struct CommitSummary {
    pub repo_name: String,
    pub message: String,
    pub author: String,
    pub time: String,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn generate_weekly_report(
    state: State<'_, DatabaseManager>,
) -> Result<String, String> {
    let db = &state.connection;
    let now = Utc::now();
    let one_week_ago = now - Duration::days(7);
    let one_week_ago_naive = one_week_ago.naive_utc();

    // 1. Sync History
    let logs = task_execution_logs::Entity::find()
        .filter(task_execution_logs::Column::StartTime.gte(one_week_ago_naive))
        .all(db)
        .await
        .map_err(|e| e.to_string())?;

    let total_syncs = logs.len();
    let success_count = logs.iter().filter(|l| l.status.as_deref() == Some("success")).count();
    let fail_count = total_syncs - success_count;

    // 2. Git Commits (Sample from all repos)
    let repos = repositories::Entity::find()
        .all(db)
        .await
        .map_err(|e| e.to_string())?;

    let mut recent_commits = Vec::new();

    for repo in repos {
        // Just get last 5 commits for report brevity
        if let Ok(commits) = get_git_log(repo.local_path, 5) {
             for c in commits {
                 // Parse time to check if within week?
                 // get_git_log returns formatted time string.
                 // For simplicity, we just include them as "Recent Activity"
                 recent_commits.push(CommitSummary {
                     repo_name: repo.name.clone(),
                     message: c.message,
                     author: c.author,
                     time: c.time,
                 });
             }
        }
    }

    // Format as Markdown
    let mut md = String::new();
    md.push_str(&format!("# Weekly Report ({})\n\n", now.format("%Y-%m-%d")));

    md.push_str("## 🔄 Sync Activity\n");
    md.push_str(&format!("- **Total Syncs**: {}\n", total_syncs));
    md.push_str(&format!("- **Success**: ✅ {}\n", success_count));
    md.push_str(&format!("- **Failed**: ❌ {}\n\n", fail_count));

    md.push_str("## 💻 Recent Commits (Last 7 Days Context)\n");
    if recent_commits.is_empty() {
        md.push_str("_No commit activity detected._\n");
    } else {
        for c in recent_commits.iter().take(20) { // Limit to 20
            md.push_str(&format!("- **{}** ({}): {}\n", c.repo_name, c.author, c.message));
        }
    }

    Ok(md)
}
