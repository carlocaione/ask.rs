use thiserror::Error;

#[derive(Error, Debug)]
pub enum AskError {
    #[error("Failed to retrieve content text from LLM reply")]
    AnswerNotFound,
    #[error("API returned error: {status} - {message}")]
    ApiError { status: u16, message: String },
    #[error("Invalid JSON response from API: {0}")]
    JsonParsingError(String),
    #[error("Request timeout - the API took too long to respond")]
    Timeout,
    #[error("Rate limit exceeded - please try again later")]
    RateLimited,
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    ClipboardError(#[from] arboard::Error),
    #[error("Key '{0}' not provided")]
    KeyMissing(String),
    #[error(transparent)]
    ParsingError(#[from] inquire::InquireError),
}
