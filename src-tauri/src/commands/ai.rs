use crate::database::manager::DatabaseManager;
use crate::core::ai_service::AIService;
use tauri::State;

#[tauri::command(rename_all = "snake_case")]
pub async fn generate_ai_response(
    system_prompt: Option<String>,
    user_prompt: String,
    state: State<'_, DatabaseManager>,
) -> Result<String, String> {
    AIService::chat_completion(&state, system_prompt, user_prompt)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn generate_release_notes(
    diff_summary: String,
    state: State<'_, DatabaseManager>,
) -> Result<String, String> {
    let system_prompt = r#"You are a helpful assistant that generates release notes from git diff summaries.
Format the output in Markdown.
Categorize changes into 'Features', 'Fixes', 'Refactoring', 'Documentation'.
Keep it concise."#.to_string();

    AIService::chat_completion(&state, Some(system_prompt), diff_summary)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn explain_diff(
    diff_content: String,
    state: State<'_, DatabaseManager>,
) -> Result<String, String> {
    let system_prompt = r#"You are an expert code reviewer.
Explain the following code diff in simple terms.
Focus on the intent and potential impact.
Highlight any risks."#.to_string();

    AIService::chat_completion(&state, Some(system_prompt), diff_content)
        .await
        .map_err(|e| e.to_string())
}
