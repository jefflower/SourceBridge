use crate::database::entities::{repo_groups, repositories};
use crate::database::manager::DatabaseManager;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tauri::{Emitter, State};
use uuid::Uuid;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScannedRepo {
    pub path: String,
    pub name: String,
    pub relative_path: String,
    pub remote_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum GroupingMode {
    Path,
    GitUrl,
}

#[derive(Serialize, Clone)]
pub struct ImportProgress {
    pub current: usize,
    pub total: usize,
    pub status: String,
}

// 功能：递归扫描本地文件夹，查找包含 .git 的目录
// 参数：root_path - 根目录路径
// 返回：扫描到的仓库列表
#[tauri::command(rename_all = "snake_case")]
pub async fn scan_local_repos(root_path: String) -> Result<Vec<ScannedRepo>, String> {
    let root = Path::new(&root_path);
    if !root.exists() || !root.is_dir() {
        return Err("无效的根目录".to_string());
    }

    let root_path_str = root.to_string_lossy().to_string();

    // 使用 spawn_blocking 在单独的线程中执行耗时的扫描操作
    let scanned_repos = tokio::task::spawn_blocking(move || {
        let mut results = Vec::new();
        let walker = WalkDir::new(&root_path_str).follow_links(true).into_iter();

        for entry in walker.filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            // 跳过常见的非源码目录以提高扫描速度
            if name == "node_modules"
                || name == "target"
                || name == ".trash"
                || name == "dist"
                || name == "build"
            {
                return false;
            }
            true
        }) {
            if let Ok(entry) = entry {
                if entry.file_type().is_dir() && entry.file_name() == ".git" {
                    // 发现 .git 目录，其父目录即为仓库根目录
                    if let Some(repo_path) = entry.path().parent() {
                        let absolute_path = repo_path.to_string_lossy().to_string();
                        let repo_name = repo_path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();

                        // 尝试获取 git 远程地址
                        let remote_url = if let Ok(repo) = git2::Repository::open(repo_path) {
                            repo.find_remote("origin")
                                .ok()
                                .and_then(|remote| remote.url().map(|u| u.to_string()))
                                .or_else(|| {
                                    // 如果没有 origin，尝试获取第一个远程仓库
                                    repo.remotes().ok().and_then(|remotes| {
                                        remotes.get(0).and_then(|name| {
                                            repo.find_remote(name).ok().and_then(|remote| {
                                                remote.url().map(|u| u.to_string())
                                            })
                                        })
                                    })
                                })
                        } else {
                            None
                        };

                        // 计算相对路径
                        // 如果 strip_prefix 失败，则使用仓库名称作为回退
                        let relative_path = match repo_path.strip_prefix(&root_path_str) {
                            Ok(p) => p.to_string_lossy().to_string(),
                            Err(_) => repo_name.clone(),
                        };

                        results.push(ScannedRepo {
                            path: absolute_path,
                            name: repo_name,
                            relative_path,
                            remote_url,
                        });
                    }
                }
            }
        }
        results
    })
    .await
    .map_err(|e| e.to_string())?;

    Ok(scanned_repos)
}

// 功能：将扫描到的仓库导入数据库，自动创建分组
// 参数：_root_path - 根路径（暂未使用），repos - 待导入的仓库列表
// 返回：成功消息
#[tauri::command(rename_all = "snake_case")]
pub async fn import_scanned_repos(
    _root_path: String,
    repos: Vec<ScannedRepo>,
    grouping_mode: GroupingMode,
    window: tauri::Window,
    state: State<'_, DatabaseManager>,
) -> Result<String, String> {
    let db = &state.connection;
    let total = repos.len();

    // 开启事务
    let txn = db.begin().await.map_err(|e| e.to_string())?;

    let mut created_count = 0;

    // 分组缓存：路径字符串 -> 分组ID
    // 用于在当前事务中避免重复查询
    let mut group_cache: HashMap<String, String> = HashMap::new();

    for (index, repo) in repos.into_iter().enumerate() {
        // 发送进度
        let _ = window.emit(
            "import_progress",
            ImportProgress {
                current: index + 1,
                total,
                status: repo.name.clone(),
            },
        );

        // 1. 创建或查找分组
        let segments = match grouping_mode {
            GroupingMode::Path => {
                let repo_path = Path::new(&repo.relative_path);
                repo_path
                    .parent()
                    .map(|p| {
                        p.components()
                            .filter_map(|c| {
                                let s = c.as_os_str().to_string_lossy().to_string();
                                if s.is_empty() || s == "." {
                                    None
                                } else {
                                    Some(s)
                                }
                            })
                            .collect::<Vec<String>>()
                    })
                    .unwrap_or_default()
            }
            GroupingMode::GitUrl => {
                if let Some(url) = &repo.remote_url {
                    parse_git_url_segments(url)
                } else {
                    // 没有 URL 时回退到路径
                    let repo_path = Path::new(&repo.relative_path);
                    repo_path
                        .parent()
                        .map(|p| {
                            p.components()
                                .filter_map(|c| {
                                    let s = c.as_os_str().to_string_lossy().to_string();
                                    if s.is_empty() || s == "." {
                                        None
                                    } else {
                                        Some(s)
                                    }
                                })
                                .collect::<Vec<String>>()
                        })
                        .unwrap_or_default()
                }
            }
        };

        let mut current_parent_id: Option<String> = None;
        let mut current_path_stack = String::new();

        for segment in segments {
            if !current_path_stack.is_empty() {
                current_path_stack.push('/');
            }
            current_path_stack.push_str(&segment);

            let path_key = current_path_stack.clone();

            if let Some(id) = group_cache.get(&path_key) {
                current_parent_id = Some(id.clone());
            } else {
                // 查询数据库
                let existing_group = repo_groups::Entity::find()
                    .filter(repo_groups::Column::Name.eq(&segment))
                    .filter(match &current_parent_id {
                        Some(pid) => repo_groups::Column::ParentId.eq(pid),
                        None => repo_groups::Column::ParentId.is_null(),
                    })
                    .one(&txn)
                    .await
                    .map_err(|e| e.to_string())?;

                if let Some(group) = existing_group {
                    current_parent_id = Some(group.id.clone());
                    group_cache.insert(path_key, group.id);
                } else {
                    // 创建新分组
                    let new_id = Uuid::new_v4().to_string();
                    let new_group = repo_groups::ActiveModel {
                        id: Set(new_id.clone()),
                        name: Set(segment),
                        parent_id: Set(current_parent_id.clone()),
                        sort_order: Set(0),
                    };
                    new_group.insert(&txn).await.map_err(|e| e.to_string())?;

                    current_parent_id = Some(new_id.clone());
                    group_cache.insert(path_key, new_id);
                }
            }
        }

        // 2. 创建或更新仓库
        // 检查是否已存在
        let existing_repo = repositories::Entity::find()
            .filter(repositories::Column::LocalPath.eq(&repo.path))
            .one(&txn)
            .await
            .map_err(|e| e.to_string())?;

        if existing_repo.is_none() {
            // 创建新仓库
            let new_repo_id = Uuid::new_v4().to_string();
            let new_repo = repositories::ActiveModel {
                id: Set(new_repo_id),
                group_id: Set(current_parent_id), // 关联到最后一个找到或创建的分组
                name: Set(repo.name),
                local_path: Set(repo.path),
                remote_url: Set(repo.remote_url),
                auth_type: Set("none".to_string()),
                created_at: Set(chrono::Utc::now().naive_utc()),
                ..Default::default()
            };
            new_repo.insert(&txn).await.map_err(|e| e.to_string())?;
            created_count += 1;
        }
    }

    txn.commit().await.map_err(|e| e.to_string())?;

    Ok(format!("成功导入 {} 个仓库", created_count))
}

fn parse_git_url_segments(url: &str) -> Vec<String> {
    // 移除 .git 后缀
    let clean_url = url.trim_end_matches(".git");

    // 识别是 SSH 格式还是 HTTP 格式
    if clean_url.contains('@') && clean_url.contains(':') {
        // git@github.com:owner/repo
        let parts: Vec<&str> = clean_url.split(':').collect();
        if parts.len() < 2 {
            return vec![];
        }

        let host_part = parts[0]; // git@github.com
        let path_part = parts[1]; // owner/repo

        let host = host_part.split('@').nth(1).unwrap_or(host_part);
        let mut segments = vec![host.to_string()];
        segments.extend(path_part.split('/').map(|s| s.to_string()));

        // 移除最后一个片段（仓库名本身）
        if segments.len() > 0 {
            segments.pop();
        }
        segments
    } else {
        // https://github.com/owner/repo
        let parts: Vec<&str> = clean_url.split("://").collect();
        let path = if parts.len() > 1 { parts[1] } else { parts[0] };

        let mut segments: Vec<String> = path.split('/').map(|s| s.to_string()).collect();
        // 移除最后一个片段（仓库名本身）
        if segments.len() > 0 {
            segments.pop();
        }
        segments
    }
}
