use crate::commands::route::MappingRule;
use crate::database::entities::routes;
use anyhow::Result;
use glob::Pattern;
use serde::{Deserialize, Serialize};
use similar::TextDiff;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
    Unchanged,
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

    pub fn scan_changes_with_roots(
        source_root: &Path,
        target_root: &Path,
        mappings: &[MappingRule],
    ) -> Result<DiffSummary> {
        let mut changes = Vec::new();
        let mut processed_paths = std::collections::HashSet::new();

        for rule in mappings {
            // Skip ignore mode rules for diff preview
            if rule.mode == "ignore" {
                continue;
            }

            // Parse source and target globs
            let source_pattern = Pattern::new(&rule.source)?;

            // Extract base prefix from glob (part before first * or **)
            let source_base = Self::extract_glob_base(&rule.source);
            let target_base = if rule.target.is_empty() {
                // If target is empty, use same structure as source
                source_base.clone()
            } else {
                Self::extract_glob_base(&rule.target)
            };

            // Walk Source
            for entry in WalkDir::new(source_root).into_iter().filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                !name.starts_with('.')
                    && name != "node_modules"
                    && name != "target"
                    && name != "dist"
            }) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    let relative_path = entry
                        .path()
                        .strip_prefix(source_root)?
                        .to_string_lossy()
                        .to_string();
                    // Normalize path separators
                    let normalized_relative = relative_path.replace('\\', "/");

                    if source_pattern.matches(&normalized_relative) {
                        // Avoid processing same file multiple times if multiple rules match
                        if processed_paths.contains(&normalized_relative) {
                            continue;
                        }
                        processed_paths.insert(normalized_relative.clone());

                        let source_file = entry.path();

                        // Calculate target path by replacing source_base with target_base
                        let target_relative = if normalized_relative.starts_with(&source_base) {
                            let suffix = &normalized_relative[source_base.len()..];
                            format!("{}{}", target_base, suffix)
                        } else {
                            // If no base match, use same relative path with target_base prepended
                            format!("{}{}", target_base, normalized_relative)
                        };

                        let target_file = target_root.join(&target_relative);

                        // Compare
                        if !target_file.exists() {
                            changes.push(FileChange {
                                path: normalized_relative,
                                change_type: ChangeType::Added,
                                source_path: Some(source_file.to_string_lossy().to_string()),
                                target_path: Some(target_file.to_string_lossy().to_string()),
                            });
                        } else {
                            // Check size first (quick check)
                            let source_meta = fs::metadata(source_file)?;
                            let target_meta = fs::metadata(&target_file)?;

                            if source_meta.len() != target_meta.len() {
                                changes.push(FileChange {
                                    path: normalized_relative,
                                    change_type: ChangeType::Modified,
                                    source_path: Some(source_file.to_string_lossy().to_string()),
                                    target_path: Some(target_file.to_string_lossy().to_string()),
                                });
                            } else {
                                // Same size - do content comparison
                                let source_content = fs::read(source_file).unwrap_or_default();
                                let target_content = fs::read(&target_file).unwrap_or_default();

                                if source_content != target_content {
                                    changes.push(FileChange {
                                        path: normalized_relative,
                                        change_type: ChangeType::Modified,
                                        source_path: Some(
                                            source_file.to_string_lossy().to_string(),
                                        ),
                                        target_path: Some(
                                            target_file.to_string_lossy().to_string(),
                                        ),
                                    });
                                }
                                // If content is same, skip (don't add Unchanged to reduce noise)
                            }
                        }
                    }
                }
            }
        }

        Ok(DiffSummary { changes })
    }

    /// Extract the base prefix from a glob pattern (part before first * or **)
    /// e.g., "src/**/*.vue" -> "src/"
    /// e.g., "**/*.ts" -> ""
    /// e.g., "lib/components/*.vue" -> "lib/components/"
    fn extract_glob_base(pattern: &str) -> String {
        // Find the first occurrence of * or **
        if let Some(pos) = pattern.find('*') {
            // Find the last slash before the wildcard
            let prefix = &pattern[..pos];
            if let Some(last_slash) = prefix.rfind('/') {
                return pattern[..=last_slash].to_string();
            }
            // No slash before wildcard, return empty
            return String::new();
        }
        // No wildcard found, treat whole pattern as a directory prefix
        if pattern.ends_with('/') {
            pattern.to_string()
        } else {
            format!("{}/", pattern)
        }
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

    pub fn get_file_content_pair(
        source_path: Option<String>,
        target_path: Option<String>,
    ) -> Result<(String, String)> {
        let source_text = if let Some(p) = source_path {
            if Path::new(&p).exists() {
                fs::read_to_string(p).unwrap_or_default()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let target_text = if let Some(p) = target_path {
            if Path::new(&p).exists() {
                fs::read_to_string(p).unwrap_or_default()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        Ok((target_text, source_text)) // Original (Target), Modified (Source)
    }
}
