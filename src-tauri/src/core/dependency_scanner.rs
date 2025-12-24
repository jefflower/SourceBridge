use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::{Result, anyhow};
use semver::Version;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DependencyWarning {
    pub package_name: String,
    pub source_version: String,
    pub target_version: String,
    pub severity: String, // "Major", "Minor", "Patch"
}

pub struct DependencyScanner;

impl DependencyScanner {
    pub fn scan(source_path: &Path, target_path: &Path) -> Result<Vec<DependencyWarning>> {
        let mut warnings = Vec::new();

        // Check package.json
        if let Ok(w) = Self::check_package_json(source_path, target_path) {
            warnings.extend(w);
        }

        // Check Cargo.toml (simplified)
        // Implementing full Cargo.toml parsing might require `toml` crate and traversing workspaces.
        // For now, let's focus on package.json as per example or add basic Cargo support.

        Ok(warnings)
    }

    fn check_package_json(source: &Path, target: &Path) -> Result<Vec<DependencyWarning>> {
        let source_pkg = source.join("package.json");
        let target_pkg = target.join("package.json");

        if !source_pkg.exists() || !target_pkg.exists() {
            return Ok(vec![]);
        }

        let source_json: serde_json::Value = serde_json::from_str(&fs::read_to_string(source_pkg)?)?;
        let target_json: serde_json::Value = serde_json::from_str(&fs::read_to_string(target_pkg)?)?;

        let mut warnings = Vec::new();

        let deps_keys = ["dependencies", "devDependencies"];

        for key in deps_keys {
             if let (Some(s_deps), Some(t_deps)) = (source_json.get(key), target_json.get(key)) {
                 if let (Some(s_obj), Some(t_obj)) = (s_deps.as_object(), t_deps.as_object()) {
                     for (pkg, s_ver_val) in s_obj {
                         if let Some(t_ver_val) = t_obj.get(pkg) {
                             let s_ver = s_ver_val.as_str().unwrap_or("");
                             let t_ver = t_ver_val.as_str().unwrap_or("");

                             if s_ver != t_ver {
                                 // Analyze severity
                                 let severity = Self::analyze_severity(s_ver, t_ver);
                                 // Only warn on Major difference?
                                 // Plan says: "Return SemanticWarning if Major versions differ."
                                 if severity == "Major" {
                                     warnings.push(DependencyWarning {
                                         package_name: pkg.clone(),
                                         source_version: s_ver.to_string(),
                                         target_version: t_ver.to_string(),
                                         severity,
                                     });
                                 }
                             }
                         }
                     }
                 }
             }
        }

        Ok(warnings)
    }

    fn analyze_severity(v1: &str, v2: &str) -> String {
        // Clean versions (remove ^, ~, etc) for semver parsing
        fn clean(v: &str) -> &str {
            v.trim_start_matches(|c| c == '^' || c == '~' || c == '=' || c == 'v')
        }

        let ver1 = Version::parse(clean(v1));
        let ver2 = Version::parse(clean(v2));

        match (ver1, ver2) {
            (Ok(a), Ok(b)) => {
                if a.major != b.major {
                    "Major".to_string()
                } else if a.minor != b.minor {
                    "Minor".to_string()
                } else if a.patch != b.patch {
                    "Patch".to_string()
                } else {
                    "Unknown".to_string()
                }
            },
            _ => "Unknown".to_string() // Non-semver string difference
        }
    }
}
