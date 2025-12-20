use anyhow::Result;
use std::fs;
use std::path::Path;
use crate::database::entities::{routes, repositories};
use crate::database::manager::DatabaseManager;
use crate::core::diff::{DiffEngine, ChangeType, DiffSummary};
use crate::commands::route::MappingRule;
use sea_orm::{EntityTrait, ModelTrait};

pub struct SyncEngine;

#[derive(Debug)]
pub struct SyncResult {
    pub success_count: usize,
    pub fail_count: usize,
    pub logs: String,
}

impl SyncEngine {
    pub async fn execute_sync(route_id: &str, db_manager: &DatabaseManager) -> Result<SyncResult> {
        let db = &db_manager.connection;

        // 1. Fetch Route
        let route = routes::Entity::find_by_id(route_id).one(db).await?
            .ok_or(anyhow::anyhow!("Route not found"))?;

        // 2. Fetch Repos
        let source_repo = repositories::Entity::find_by_id(route.main_repo_id.clone().ok_or(anyhow::anyhow!("No source"))?).one(db).await?
            .ok_or(anyhow::anyhow!("Source repo not found"))?;
        let target_repo = repositories::Entity::find_by_id(route.slave_repo_id.clone().ok_or(anyhow::anyhow!("No target"))?).one(db).await?
            .ok_or(anyhow::anyhow!("Target repo not found"))?;

        // 3. Parse Mappings
        let mappings_json = route.mappings.ok_or(anyhow::anyhow!("No mappings"))?;
        let mappings: Vec<MappingRule> = serde_json::from_str(&mappings_json)?;

        let source_root = Path::new(&source_repo.local_path);
        let target_root = Path::new(&target_repo.local_path);

        // 4. Scan
        let summary = DiffEngine::scan_changes_with_roots(source_root, target_root, &mappings)?;

        // 5. Execute
        let mut success = 0;
        let mut fail = 0;
        let mut logs = String::new();

        logs.push_str(&format!("Syncing Route: {}\n", route.name));

        for change in summary.changes {
            match change.change_type {
                ChangeType::Added | ChangeType::Modified => {
                    if let (Some(src), Some(dst)) = (change.source_path, change.target_path) {
                        let src_path = Path::new(&src);
                        let dst_path = Path::new(&dst);

                        if let Some(parent) = dst_path.parent() {
                            if !parent.exists() {
                                if let Err(e) = fs::create_dir_all(parent) {
                                    logs.push_str(&format!("[ERR] Failed to create dir {:?}: {}\n", parent, e));
                                    fail += 1;
                                    continue;
                                }
                            }
                        }

                        match fs::copy(src_path, dst_path) {
                            Ok(_) => {
                                logs.push_str(&format!("[OK] Copied {:?} -> {:?}\n", src_path, dst_path));
                                success += 1;
                            },
                            Err(e) => {
                                logs.push_str(&format!("[ERR] Copy failed {:?} -> {:?}: {}\n", src_path, dst_path, e));
                                fail += 1;
                            }
                        }
                    }
                },
                ChangeType::Deleted => {
                    // Implement deletion if configured
                    logs.push_str(&format!("[SKIP] Deletion detected but not enabled for {:?}\n", change.path));
                },
                ChangeType::Unchanged => {
                    // logs.push_str(&format!("[SKIP] Unchanged {:?}\n", change.path));
                }
            }
        }

        logs.push_str(&format!("Sync Completed. Success: {}, Failed: {}\n", success, fail));

        Ok(SyncResult {
            success_count: success,
            fail_count: fail,
            logs
        })
    }
}
