use git2::{Repository, Status, StatusOptions};
use serde::{Deserialize, Serialize};
use crate::database::entities::repositories;
use crate::database::manager::DatabaseManager;
use sea_orm::{EntityTrait};
use tauri::State;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::core::dependency_scanner::DependencyScanner;
use crate::database::entities::routes;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoStatus {
    pub id: String,
    pub name: String,
    pub path: String,
    pub status: String, // "Clean", "Uncommitted", "Behind", "Ahead", "Diverged", "Error"
    pub short_summary: String, // e.g., "3↓ 1↑", "2 M", "Clean"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoStatusMap {
    pub statuses: HashMap<String, RepoStatus>,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn scan_dependencies(
    source_path: String,
    target_path: String,
) -> Result<Vec<crate::core::dependency_scanner::DependencyWarning>, String> {
    let source = Path::new(&source_path);
    let target = Path::new(&target_path);

    DependencyScanner::scan(source, target)
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_repos_status(
    state: State<'_, DatabaseManager>,
) -> Result<RepoStatusMap, String> {
    let db = &state.connection;

    // Fetch all repos
    let repos = repositories::Entity::find()
        .all(db)
        .await
        .map_err(|e| e.to_string())?;

    let mut results = HashMap::new();
    let results_mutex = Arc::new(Mutex::new(&mut results));

    // Parallel processing using tokio::spawn could be done,
    // but for simplicity and since we are in an async command, we can just iterate asyncly if we had async git ops.
    // git2 is synchronous. Using `tokio::task::spawn_blocking` is better.

    let mut handles = vec![];

    for repo in repos {
        let repo_path = repo.local_path.clone();
        let repo_id = repo.id.clone();
        let repo_name = repo.name.clone();

        let handle = tokio::task::spawn_blocking(move || {
            check_repo_status(repo_id, repo_name, repo_path)
        });
        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok(status) => {
                results.insert(status.id.clone(), status);
            },
            Err(e) => {
                eprintln!("Task join error: {}", e);
            }
        }
    }

    Ok(RepoStatusMap { statuses: results })
}

fn check_repo_status(id: String, name: String, path_str: String) -> RepoStatus {
    let path = Path::new(&path_str);
    if !path.exists() {
        return RepoStatus {
            id,
            name,
            path: path_str,
            status: "Error".to_string(),
            short_summary: "Path missing".to_string(),
        };
    }

    let repo = match Repository::open(path) {
        Ok(r) => r,
        Err(e) => return RepoStatus {
            id,
            name,
            path: path_str,
            status: "Error".to_string(),
            short_summary: e.to_string(),
        },
    };

    // Check status (modified files)
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);

    let statuses = match repo.statuses(Some(&mut opts)) {
        Ok(s) => s,
        Err(e) => return RepoStatus {
            id,
            name,
            path: path_str,
            status: "Error".to_string(),
            short_summary: e.to_string(),
        },
    };

    let has_changes = !statuses.is_empty();
    let change_count = statuses.len();

    // Check ahead/behind
    let (ahead, behind) = match get_ahead_behind(&repo) {
        Ok((a, b)) => (a, b),
        Err(_) => (0, 0),
    };

    let status_str = if ahead > 0 && behind > 0 {
        "Diverged"
    } else if ahead > 0 {
        "Ahead"
    } else if behind > 0 {
        "Behind"
    } else if has_changes {
        "Uncommitted"
    } else {
        "Clean"
    };

    let mut summary_parts = vec![];
    if behind > 0 {
        summary_parts.push(format!("{}↓", behind));
    }
    if ahead > 0 {
        summary_parts.push(format!("{}↑", ahead));
    }
    if has_changes {
        summary_parts.push(format!("{} M", change_count));
    }

    let short_summary = if summary_parts.is_empty() {
        "✓".to_string()
    } else {
        summary_parts.join(" ")
    };

    RepoStatus {
        id,
        name,
        path: path_str,
        status: status_str.to_string(),
        short_summary,
    }
}

fn get_ahead_behind(repo: &Repository) -> Result<(usize, usize), git2::Error> {
    let head = repo.head()?;
    let head_oid = head.target().ok_or_else(|| git2::Error::from_str("No HEAD target"))?;

    let upstream = match repo.branch_upstream_name(head.name().unwrap_or("HEAD")) {
        Ok(buf) => {
            let upstream_name = buf.as_str().unwrap_or("");
            repo.find_reference(upstream_name)?
        },
        Err(_) => return Ok((0, 0)), // No upstream
    };

    let upstream_oid = upstream.target().ok_or_else(|| git2::Error::from_str("No upstream target"))?;

    let (ahead, behind) = repo.graph_ahead_behind(head_oid, upstream_oid)?;
    Ok((ahead, behind))
}
