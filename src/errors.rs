use thiserror::Error;

#[derive(Error, Debug)]
pub enum AskError {
    #[error("Failed to retrieve content text from LLM reply")]
    AnswerNotFound,
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
