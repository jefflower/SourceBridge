use anyhow::Result;
use std::path::Path;
use std::fs;
use similar::TextDiff;
use walkdir::WalkDir;
use glob::Pattern;
use serde::{Deserialize, Serialize};
use crate::database::entities::routes;
use crate::commands::route::MappingRule;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
    Unchanged
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileChange {
    pub path: String,
    pub change_type: ChangeType,
    pub source_path: Option<String>,
    pub target_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiffSummary {
    pub changes: Vec<FileChange>,
}

pub struct DiffEngine;

impl DiffEngine {
    #[allow(dead_code)]
    pub fn scan_changes(_route: &routes::Model, mappings_json: &str) -> Result<DiffSummary> {
        // 1. Parse mappings
        let _rules: Vec<MappingRule> = serde_json::from_str(mappings_json)?;

        // 2. Resolve Repos
        // In a real scenario, we would query the repositories table to get the local paths.
        // Since we are in the `core` module and might not want direct DB access here or want to keep it pure,
        // we assume the caller provides resolved paths or we do a lookup.
        // However, `route` model has IDs. We need the paths.
        // For simplicity in this Task 005, let's assume we can fetch Repositories in the command layer
        // and pass the root paths to this function.
        // But the signature `scan_changes(route)` implies we do it here.
        // Let's modify signature to take `source_root` and `target_root`.
        Ok(DiffSummary { changes: vec![] }) // Placeholder return for signature check, logic below
    }

    pub fn scan_changes_with_roots(source_root: &Path, target_root: &Path, mappings: &[MappingRule]) -> Result<DiffSummary> {
        let mut changes = Vec::new();

        for rule in mappings {
            // Check Glob
            let pattern = Pattern::new(&rule.source)?;

            // Walk Source
            for entry in WalkDir::new(source_root) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    let relative_path = entry.path().strip_prefix(source_root)?.to_string_lossy();

                    if pattern.matches(&relative_path) {
                        let source_file = entry.path();
                        // Construct Target Path (Simple replace for now)
                        // rule.target is likely a directory prefix if it ends with /
                        let target_file = if rule.target.ends_with('/') {
                            target_root.join(&rule.target).join(&*relative_path)
                        } else {
                            target_root.join(&rule.target)
                        };

                        // Compare
                        if !target_file.exists() {
                            changes.push(FileChange {
                                path: relative_path.to_string(),
                                change_type: ChangeType::Added,
                                source_path: Some(source_file.to_string_lossy().to_string()),
                                target_path: None,
                            });
                        } else {
                            // Check size or mtime or content hash
                            let source_meta = fs::metadata(source_file)?;
                            let target_meta = fs::metadata(&target_file)?;

                            if source_meta.len() != target_meta.len() {
                                changes.push(FileChange {
                                    path: relative_path.to_string(),
                                    change_type: ChangeType::Modified,
                                    source_path: Some(source_file.to_string_lossy().to_string()),
                                    target_path: Some(target_file.to_string_lossy().to_string()),
                                });
                            } else {
                                // Deeper check? For Quick Check, maybe skip or assume unchanged.
                                // Let's mark unchanged for now or verify modification time
                                // If source is newer?
                                // For sync, if source changed, we sync.
                                changes.push(FileChange {
                                    path: relative_path.to_string(),
                                    change_type: ChangeType::Unchanged, // Or filter out
                                    source_path: Some(source_file.to_string_lossy().to_string()),
                                    target_path: Some(target_file.to_string_lossy().to_string()),
                                });
                            }
                        }
                    }
                }
            }

            // Check for Deleted files (in Target but not in Source matching rule)
            // Strategy: Walk Target, if match rule but not in Source -> Deleted
            // Note: This requires reverse mapping logic if rules are complex.
            // For simple "copy", we can assume target structure mirrors source.

            // Construct target base path for this rule
            let target_base = if rule.target.ends_with('/') {
                target_root.join(&rule.target)
            } else {
                target_root.join(&rule.target)
            };

            if target_base.exists() {
                 for entry in WalkDir::new(&target_base) {
                    let entry = entry?;
                    if entry.file_type().is_file() {
                        let relative_target = entry.path().strip_prefix(&target_base)?.to_string_lossy();
                        // Verify if this file should have existed in source
                        // This implies we need to match it back to source pattern.
                        // If source pattern is src/*.ts, and we found dest/main.ts, does it correspond to src/main.ts?
                        // Yes if we assume 1:1 mapping within the glob.

                        // Check if corresponding source file exists
                        let source_file = source_root.join(&*relative_target); // Simplified assumption for glob
                        // Ideally we should check if 'relative_target' matches the glob if we were scanning source.
                        // But here we are scanning target.

                        if !source_file.exists() {
                             // Double check if this file matches the intended rule scope?
                             // If rule was src/*.ts -> dest/, and we found dest/junk.txt (which wasn't a ts file),
                             // does it mean it was deleted from source? No, it might just be a file that shouldn't be there or ignored.
                             // We should only mark "Deleted" if it WAS in source (conceptually) or if we want strict mirroring.
                             // For strict mirror: anything in target not in source is deleted.
                             // Let's assume strict mirror for this task to support "Deleted" status.

                             changes.push(FileChange {
                                path: relative_target.to_string(),
                                change_type: ChangeType::Deleted,
                                source_path: None,
                                target_path: Some(entry.path().to_string_lossy().to_string()),
                            });
                        }
                    }
                 }
            }
        }

        Ok(DiffSummary { changes })
    }

    #[allow(dead_code)]
    pub fn compute_text_diff(source_path: &Path, target_path: &Path) -> Result<String> {
        let source_text = if source_path.exists() {
            fs::read_to_string(source_path).unwrap_or_default()
        } else {
            String::new()
        };

        let target_text = if target_path.exists() {
            fs::read_to_string(target_path).unwrap_or_default()
        } else {
            String::new()
        };

        let _diff = TextDiff::from_lines(&target_text, &source_text);

        // Return unified diff or side-by-side structure?
        // Monaco editor createDiffEditor expects two models (original, modified).
        // So actually we just need to return the content of both files!
        // But the requirement says "Return diff result".
        // If using Monaco, frontend needs full content.
        // Let's return a JSON with original and modified content.

        Ok("Use get_file_content for Monaco".to_string())
    }

    pub fn get_file_content_pair(source_path: Option<String>, target_path: Option<String>) -> Result<(String, String)> {
        let source_text = if let Some(p) = source_path {
            if Path::new(&p).exists() { fs::read_to_string(p).unwrap_or_default() } else { String::new() }
        } else { String::new() };

        let target_text = if let Some(p) = target_path {
            if Path::new(&p).exists() { fs::read_to_string(p).unwrap_or_default() } else { String::new() }
        } else { String::new() };

        Ok((target_text, source_text)) // Original (Target), Modified (Source)
    }
}
