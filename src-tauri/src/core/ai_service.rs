use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::time::Duration;
use crate::database::manager::DatabaseManager;
use crate::database::entities::settings;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use anyhow::{Result, anyhow};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub message: Message,
}

pub struct AIService;

impl AIService {
    pub async fn chat_completion(
        db: &DatabaseManager,
        system_prompt: Option<String>,
        user_prompt: String,
    ) -> Result<String> {
        let (endpoint, model, api_key) = Self::get_config(db).await?;

        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()?;

        let mut messages = Vec::new();
        if let Some(sys) = system_prompt {
            messages.push(Message {
                role: "system".to_string(),
                content: sys,
            });
        }
        messages.push(Message {
            role: "user".to_string(),
            content: user_prompt,
        });

        let payload = ChatRequest {
            model: model.clone(),
            messages,
            temperature: Some(0.7),
        };

        let mut request = client.post(format!("{}/v1/chat/completions", endpoint.trim_end_matches('/')))
            .json(&payload);

        if let Some(key) = api_key {
             if !key.is_empty() {
                request = request.header("Authorization", format!("Bearer {}", key));
             }
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("AI API Error: {}", error_text));
        }

        let chat_res: ChatResponse = response.json().await?;

        if let Some(choice) = chat_res.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow!("No response content from AI"))
        }
    }

    async fn get_config(db: &DatabaseManager) -> Result<(String, String, Option<String>)> {
        async fn get_val(db: &DatabaseManager, key: &str) -> String {
             let s = settings::Entity::find()
                .filter(settings::Column::Key.eq(key))
                .one(&db.connection)
                .await
                .ok()
                .flatten();
            s.map(|m| m.value).unwrap_or_default()
        }

        let endpoint = get_val(db, "ai_endpoint").await;
        let model = get_val(db, "ai_model").await;
        let api_key = get_val(db, "ai_api_key").await;

        let endpoint = if endpoint.is_empty() { "http://localhost:11434".to_string() } else { endpoint };
        let model = if model.is_empty() { "llama3".to_string() } else { model };
        let api_key = if api_key.is_empty() { None } else { Some(api_key) };

        Ok((endpoint, model, api_key))
    }
}
