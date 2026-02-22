use crate::config::AppConfig;
use crate::error::AppError;
use crate::llm::types::{ApiRequest, ApiResponse, ApiError};

pub struct LlmClient {
    client: reqwest::Client,
    api_key: String,
}

impl LlmClient {
    pub fn new(config: &AppConfig) -> Result<Self, AppError> {
        if config.api.anthropic_api_key.is_empty() {
            return Err(AppError::Llm("Anthropic API key not configured".to_string()));
        }
        Ok(Self {
            client: reqwest::Client::new(),
            api_key: config.api.anthropic_api_key.clone(),
        })
    }

    pub async fn send(&self, request: &ApiRequest) -> Result<ApiResponse, AppError> {
        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(request)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            let api_error: ApiError = serde_json::from_str(&body).unwrap_or(ApiError {
                error_type: None,
                error: None,
            });
            let msg = api_error
                .error
                .map(|e| e.message)
                .unwrap_or_else(|| format!("API error {}: {}", status, body));
            return Err(AppError::Llm(msg));
        }

        let api_response: ApiResponse = serde_json::from_str(&body)
            .map_err(|e| AppError::Llm(format!("Failed to parse API response: {}", e)))?;

        Ok(api_response)
    }
}
