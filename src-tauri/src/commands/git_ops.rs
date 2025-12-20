use git2::{BranchType, Repository, build::CheckoutBuilder};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct BranchInfo {
    pub name: String,
    pub is_current: bool,
    pub is_remote: bool,
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_git_branches(path: String) -> Result<Vec<BranchInfo>, String> {
    let repo_path = Path::new(&path);
    let repo = Repository::open(repo_path).map_err(|e| format!("Failed to open git repo: {}", e))?;

    let mut branches = Vec::new();

    // List local branches
    let local_branches = repo.branches(Some(BranchType::Local)).map_err(|e| e.to_string())?;
    for b in local_branches {
        let (branch, _) = b.map_err(|e| e.to_string())?;
        let name = branch.name().map_err(|e| e.to_string())?.unwrap_or("").to_string();
        let is_current = branch.is_head();

        branches.push(BranchInfo {
            name,
            is_current,
            is_remote: false,
        });
    }

    // List remote branches
    let remote_branches = repo.branches(Some(BranchType::Remote)).map_err(|e| e.to_string())?;
    for b in remote_branches {
        let (branch, _) = b.map_err(|e| e.to_string())?;
        let name = branch.name().map_err(|e| e.to_string())?.unwrap_or("").to_string();
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
    let repo = Repository::open(repo_path).map_err(|e| format!("Failed to open git repo: {}", e))?;

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
             repo.revparse_single(&remote_ref_name).map_err(|e| format!("Branch not found: {}", e))?
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
         repo.set_head(&branch_ref_name).map_err(|e| format!("Failed to set HEAD: {}", e))?;
    } else {
         // Detached head for commit/remote?
         repo.set_head_detached(obj.id()).map_err(|e| format!("Failed to detach HEAD: {}", e))?;
    }

    Ok(())
}
