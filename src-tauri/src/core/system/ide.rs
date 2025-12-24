use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeInfo {
    pub name: String,
    pub command: String, // Command to launch (e.g. 'code', 'cursor')
    pub icon: Option<String>,
}

pub fn detect_installed_ides() -> Vec<IdeInfo> {
    let mut ides = Vec::new();

    // Heuristic detection based on common paths or commands
    // In a real implementation, we would check PATH or specific OS locations.
    // For now, we simulate detection or check simple commands.

    // VS Code
    if is_command_available("code") {
        ides.push(IdeInfo {
            name: "VS Code".to_string(),
            command: "code".to_string(),
            icon: None,
        });
    }

    // Cursor
    if is_command_available("cursor") {
        ides.push(IdeInfo {
            name: "Cursor".to_string(),
            command: "cursor".to_string(),
            icon: None,
        });
    }

    // Sublime Text
    if is_command_available("subl") {
        ides.push(IdeInfo {
            name: "Sublime Text".to_string(),
            command: "subl".to_string(),
            icon: None,
        });
    }

    // IntelliJ IDEA (idea)
    if is_command_available("idea") {
        ides.push(IdeInfo {
            name: "IntelliJ IDEA".to_string(),
            command: "idea".to_string(),
            icon: None,
        });
    }

    // WebStorm (webstorm)
    if is_command_available("webstorm") {
        ides.push(IdeInfo {
            name: "WebStorm".to_string(),
            command: "webstorm".to_string(),
            icon: None,
        });
    }

    // Fallback/Default if empty? No, just return detected.

    ides
}

fn is_command_available(command: &str) -> bool {
    which::which(command).is_ok()
}
