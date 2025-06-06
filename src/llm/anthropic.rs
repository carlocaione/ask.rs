use owo_colors::OwoColorize;
use serde_json::json;

use crate::errors::AskError;

use super::{PROMPT, Provider};

const URL: &str = "https://api.anthropic.com/v1/messages";

const HV_VERSION: &str = "2023-06-01";
const DATA_MODEL: &str = "claude-opus-4-20250514";
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
        let client = reqwest::Client::new();
        let content = format!("{PROMPT} {query}");

        let json = json!({
            "model": DATA_MODEL,
            "max_tokens": DATA_MAX_TOKENS,
            "messages": [
                { "role": "user", "content": content }
            ]
        });

        Ok(client
            .post(URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", HV_VERSION)
            .header("content-type", "application/json")
            .json(&json)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    fn get_answer_from(&self, json: &serde_json::Value) -> Result<String, AskError> {
        Ok(json["content"][0]["text"]
            .as_str()
            .ok_or(AskError::AnswerNotFound)?
            .to_string())
    }

    fn get_details_from(&self, json: &serde_json::Value) {
        println!();
        println!("{}: {}", "api-key: ".bold(), &self.api_key);
        println!(
            "{}: {}",
            "model: ".bold(),
            json["model"].as_str().unwrap_or("<unknown>")
        );
        println!(
            "{}: {}",
            "input tokens: ".bold(),
            json["usage"]["input_tokens"].as_u64().unwrap_or_default()
        );
        println!(
            "{}: {}",
            "output tokens: ".bold(),
            json["usage"]["output_tokens"].as_u64().unwrap_or_default()
        )
    }
}
