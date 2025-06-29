use owo_colors::OwoColorize;
use serde_json::json;

use crate::errors::AskError;

use super::{PROMPT, Provider};

const URL: &str = "https://api.anthropic.com/v1/messages";

const HV_VERSION: &str = "2023-06-01";
const DATA_MODEL: &str = "claude-3-5-sonnet-20241022";
const DATA_MAX_TOKENS: u32 = 1024;

pub struct Anthropic {
    api_key: String,
}

impl Provider for Anthropic {
    const API_KEY_ENV: &str = "ASK_ANTHROPIC_API_KEY";

    fn new(api_key: &str) -> Self {
        Anthropic {
            api_key: api_key.to_string(),
        }
    }

    async fn do_query(&self, query: &str) -> Result<serde_json::Value, AskError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()?;
        let content = format!("{PROMPT} {query}");

        let json = json!({
            "model": DATA_MODEL,
            "max_tokens": DATA_MAX_TOKENS,
            "messages": [
                { "role": "user", "content": content }
            ]
        });

        let response = client
            .post(URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", HV_VERSION)
            .header("content-type", "application/json")
            .json(&json)
            .send()
            .await?;

        let status = response.status();

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(AskError::RateLimited);
        }

        if status == reqwest::StatusCode::REQUEST_TIMEOUT || status == reqwest::StatusCode::GATEWAY_TIMEOUT {
            return Err(AskError::Timeout);
        }

        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_else(|e| format!("Failed to read error response: {}", e));
            return Err(AskError::ApiError {
                status: status.as_u16(),
                message: error_body,
            });
        }

        response.json().await.map_err(|e| AskError::JsonParsingError(e.to_string()))
    }

    fn get_answer_from(&self, json: &serde_json::Value) -> Result<String, AskError> {
        let content = json.get("content")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|item| item.get("text"))
            .and_then(|text| text.as_str())
            .ok_or(AskError::AnswerNotFound)?;

        Ok(content.to_string())
    }

    fn get_details_from(&self, json: &serde_json::Value) {
        println!();
        println!("{}: {}", "api-key: ".bold(), &self.api_key);

        let model = json.get("model")
            .and_then(|m| m.as_str())
            .unwrap_or("<model not found>");
        println!("{}: {}", "model: ".bold(), model);

        let input_tokens = json.get("usage")
            .and_then(|u| u.get("input_tokens"))
            .and_then(|t| t.as_u64())
            .unwrap_or(0);
        println!("{}: {}", "input tokens: ".bold(), input_tokens);

        let output_tokens = json.get("usage")
            .and_then(|u| u.get("output_tokens"))
            .and_then(|t| t.as_u64())
            .unwrap_or(0);
        println!("{}: {}", "output tokens: ".bold(), output_tokens);
    }
}
