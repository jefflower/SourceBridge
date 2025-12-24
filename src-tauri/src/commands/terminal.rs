use tauri::{AppHandle, Manager, Emitter, Runtime};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use std::path::Path;
use serde::Serialize;

#[derive(Clone, Serialize)]
struct CommandLog {
    text: String,
}

#[derive(Clone, Serialize)]
struct CommandEnd {
    success: bool,
    code: Option<i32>,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn run_shell_command<R: Runtime>(
    app: AppHandle<R>,
    command: String,
    args: Vec<String>,
    cwd: Option<String>,
) -> Result<(), String> {
    let mut cmd = tokio::process::Command::new(&command);
    cmd.args(&args);

    if let Some(path) = cwd {
        cmd.current_dir(Path::new(&path));
    }

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    // Kill any existing command if needed? For now, just spawn.
    // In a real app, we might want to track the process to kill it later.

    let mut child = cmd.spawn().map_err(|e| format!("Failed to spawn command: {}", e))?;

    let stdout = child.stdout.take().expect("Failed to open stdout");
    let stderr = child.stderr.take().expect("Failed to open stderr");

    let app_handle = app.clone();
    let app_handle2 = app.clone();

    // Stream stdout
    tauri::async_runtime::spawn(async move {
        let mut reader = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            let _ = app_handle.emit("cmd://log", CommandLog { text: line + "\n" });
        }
    });

    // Stream stderr
    tauri::async_runtime::spawn(async move {
        let mut reader = BufReader::new(stderr).lines();
        while let Ok(Some(line)) = reader.next_line().await {
             let _ = app_handle2.emit("cmd://log", CommandLog { text: line + "\n" });
        }
    });

    // Wait for exit
    tauri::async_runtime::spawn(async move {
        let status = child.wait().await;
        match status {
            Ok(s) => {
                let _ = app.emit("cmd://end", CommandEnd {
                    success: s.success(),
                    code: s.code()
                });
            }
            Err(_) => {
                 let _ = app.emit("cmd://end", CommandEnd {
                    success: false,
                    code: None
                });
            }
        }
    });

    Ok(())
}
