use crate::error::{AppError, Result};
use std::time::Duration;

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| AppError::HttpRequest(e.to_string()))?;

        Ok(Self { client })
    }

    pub async fn fetch_html(&self, url: &str) -> Result<String> {
        tracing::info!("Fetching HTML from {}", url);

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| AppError::HttpRequest(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            return Err(AppError::HttpRequest(format!(
                "HTTP {} for {}",
                status, url
            )));
        }

        let html = response
            .text()
            .await
            .map_err(|e| AppError::HttpRequest(e.to_string()))?;

        tracing::info!("Fetched {} bytes", html.len());
        Ok(html)
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}
