use crate::error::{AppError, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub target_url: String,
    pub aws_region: String,
    pub dynamodb_table: String,
    pub dynamodb_endpoint: Option<String>,
    pub google_calendar_id: String,
    pub google_credentials_file: Option<String>,
    pub google_credentials_base64: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            target_url: env::var("TARGET_URL")
                .map_err(|_| AppError::Config("TARGET_URL is required".to_string()))?,
            aws_region: env::var("AWS_REGION").unwrap_or_else(|_| "ap-northeast-1".to_string()),
            dynamodb_table: env::var("DYNAMODB_TABLE")
                .unwrap_or_else(|_| "rrdc-releases".to_string()),
            dynamodb_endpoint: env::var("DYNAMODB_ENDPOINT").ok().filter(|s| !s.is_empty()),
            google_calendar_id: env::var("GOOGLE_CALENDAR_ID")
                .map_err(|_| AppError::Config("GOOGLE_CALENDAR_ID is required".to_string()))?,
            google_credentials_file: env::var("GOOGLE_CREDENTIALS_FILE").ok(),
            google_credentials_base64: env::var("GOOGLE_CREDENTIALS_BASE64").ok(),
        })
    }
}
