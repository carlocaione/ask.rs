pub mod anthropic;

use anyhow::Result;

pub const PROMPT: &str = "
    You are a CLI command generator. The user will describe what they want to do in a terminal environment.

    IMPORTANT: Your response must contain ONLY the exact command to run. Do not include:
    - Explanations or descriptions
    - Markdown formatting or code blocks
    - Multiple command options
    - Any additional text

    Respond with just the raw command that can be copied and pasted directly into the terminal.
    When not specified otherwise, you can assume we are running on Linux or MacOS.

    User request: ";

pub trait Provider {
    const API_KEY_ENV: &str;

    fn new(api_key: &str) -> Self;
    async fn do_query(&self, query: &str) -> Result<serde_json::Value>;
    fn get_details_from(&self, json: &serde_json::Value);
    fn get_answer_from(&self, json: &serde_json::Value) -> Result<String>;
}
