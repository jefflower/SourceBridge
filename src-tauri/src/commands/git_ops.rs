use chrono::{TimeZone, Utc};
use git2::{build::CheckoutBuilder, BranchType, Repository};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct BranchInfo {
    pub name: String,
    pub is_current: bool,
    pub is_remote: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitInfo {
    pub id: String,
    pub author: String,
    pub time: String,
    pub message: String,
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_git_branches(path: String) -> Result<Vec<BranchInfo>, String> {
    let repo_path = Path::new(&path);
    let repo =
        Repository::open(repo_path).map_err(|e| format!("Failed to open git repo: {}", e))?;

    let mut branches = Vec::new();

    // List local branches
    let local_branches = repo
        .branches(Some(BranchType::Local))
        .map_err(|e| e.to_string())?;
    for b in local_branches {
        let (branch, _) = b.map_err(|e| e.to_string())?;
        let name = branch
            .name()
            .map_err(|e| e.to_string())?
            .unwrap_or("")
            .to_string();
        let is_current = branch.is_head();

        branches.push(BranchInfo {
            name,
            is_current,
            is_remote: false,
        });
    }

    // List remote branches
    let remote_branches = repo
        .branches(Some(BranchType::Remote))
        .map_err(|e| e.to_string())?;
    for b in remote_branches {
        let (branch, _) = b.map_err(|e| e.to_string())?;
        let name = branch
            .name()
            .map_err(|e| e.to_string())?
            .unwrap_or("")
            .to_string();
        // Remote branches are rarely "head" in the local sense, usually detached or tracking
        // But git2 `is_head` checks if HEAD points to this branch reference.
        let is_current = branch.is_head();

        branches.push(BranchInfo {
            name,
            is_current,
            is_remote: true,
        });
    }

    Ok(branches)
}

#[tauri::command(rename_all = "snake_case")]
pub fn switch_git_branch(path: String, branch: String) -> Result<(), String> {
    let repo_path = Path::new(&path);
    let repo =
        Repository::open(repo_path).map_err(|e| format!("Failed to open git repo: {}", e))?;

    // Find the branch
    // This logic handles local branches. If it's a remote branch, we might need to create a local tracking branch.
    // For now, assuming user switches to an existing local branch or we handle remote checkout simply.

    // First try to find local branch
    let branch_ref_name = format!("refs/heads/{}", branch);
    let obj = match repo.revparse_single(&branch_ref_name) {
        Ok(o) => o,
        Err(_) => {
            // Try remote?
            let remote_ref_name = format!("refs/remotes/{}", branch);
            repo.revparse_single(&remote_ref_name)
                .map_err(|e| format!("Branch not found: {}", e))?
        }
    };

    // Checkout
    // We need to detach head or set head to the branch ref.
    // If it is a local branch, we want to set HEAD to refs/heads/branchName
    // If remote, we usually checkout as detached HEAD or create a new branch.

    // Simplification: Check if it's a local branch, if so set HEAD.
    // If not, maybe just error for now (Plan says "Switch branch", usually implies local).
    // Let's assume input `branch` is the short name of a local branch for now, or full name.

    // Safe switch
    repo.checkout_tree(&obj, Some(CheckoutBuilder::new().safe()))
        .map_err(|e| format!("Checkout failed (conflict?): {}", e))?;

    if repo.find_branch(&branch, BranchType::Local).is_ok() {
        repo.set_head(&branch_ref_name)
            .map_err(|e| format!("Failed to set HEAD: {}", e))?;
    } else {
        // Detached head for commit/remote?
        repo.set_head_detached(obj.id())
            .map_err(|e| format!("Failed to detach HEAD: {}", e))?;
    }

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_git_log(path: String, count: i32) -> Result<Vec<CommitInfo>, String> {
    let repo_path = Path::new(&path);
    let repo =
        Repository::open(repo_path).map_err(|e| format!("Failed to open git repo: {}", e))?;

    let mut revwalk = repo.revwalk().map_err(|e| e.to_string())?;
    revwalk.push_head().map_err(|e| e.to_string())?;
    revwalk
        .set_sorting(git2::Sort::TIME)
        .map_err(|e| e.to_string())?;

    let mut commits = Vec::new();
    let mut i = 0;

    for id in revwalk {
        if i >= count {
            break;
        }
        let id = id.map_err(|e| e.to_string())?;
        let commit = repo.find_commit(id).map_err(|e| e.to_string())?;

        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown").to_string();

        let time_val = commit.time().seconds();
        let dt = Utc
            .timestamp_opt(time_val, 0)
            .single()
            .unwrap_or(Utc::now());
        let time_str = dt.format("%Y-%m-%d %H:%M").to_string();

        let message = commit.message().unwrap_or("").trim().to_string();
        let short_id = id.to_string().chars().take(7).collect::<String>();

        commits.push(CommitInfo {
            id: short_id,
            author: author_name,
            time: time_str,
            message,
        });
        i += 1;
    }

    Ok(commits)
}

#[tauri::command(rename_all = "snake_case")]
pub fn open_in_folder(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn open_in_terminal(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/c", "start", "cmd", "/k", &format!("cd /d {}", path)])
            .spawn()
            .map_err(|e| format!("Failed to open terminal: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(&["-a", "Terminal", &path])
            .spawn()
            .map_err(|e| format!("Failed to open terminal: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        // Try common terminal emulators
        // This is tricky on Linux as there is no standard "default terminal" command that opens in a path reliably across DEs
        // `x-terminal-emulator` is a Debian/Ubuntu alternative system.
        // `gnome-terminal`, `konsole`, `xfce4-terminal`.

        let terminals = [
            "x-terminal-emulator",
            "gnome-terminal",
            "konsole",
            "xfce4-terminal",
            "xterm",
        ];
        let mut success = false;

        for term in terminals {
            // Argument handling differs by terminal...
            // Gnome/XFCE: --working-directory
            // Konsole: --workdir
            // xterm: -e cd path? No easy way.

            let mut cmd = std::process::Command::new(term);

            if term == "gnome-terminal" || term == "xfce4-terminal" {
                cmd.args(&["--working-directory", &path]);
            } else if term == "konsole" {
                cmd.args(&["--workdir", &path]);
            } else {
                // For generic, just try spawning it, user might have to cd manually if args not supported
                // Or try passing path if it supports it
            }

            if cmd.spawn().is_ok() {
                success = true;
                break;
            }
        }

        if !success {
            return Err("Could not find a supported terminal emulator".to_string());
        }
    }
    Ok(())
}
