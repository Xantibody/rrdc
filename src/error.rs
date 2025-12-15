use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("HTTP request failed: {0}")]
    HttpRequest(String),

    #[error("HTML parsing error: {message}")]
    HtmlParse { message: String },

    #[error("Date parsing error: {input}")]
    DateParse { input: String },

    #[error("DynamoDB operation failed: {operation}")]
    DynamoDb {
        operation: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Google Calendar API error: {0}")]
    GoogleCalendar(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
