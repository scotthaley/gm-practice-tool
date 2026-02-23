use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub api: ApiConfig,
    pub models: ModelsConfig,
    pub app: AppSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub groq_api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsConfig {
    pub router: String,
    pub player: String,
    pub parameters: ModelParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParameters {
    pub router_max_tokens: u32,
    pub player_max_tokens: u32,
    pub temperature: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api: ApiConfig {
                groq_api_key: String::new(),
            },
            models: ModelsConfig {
                router: "llama-3.1-8b-instant".to_string(),
                player: "llama-3.3-70b-versatile".to_string(),
                parameters: ModelParameters {
                    router_max_tokens: 500,
                    player_max_tokens: 2000,
                    temperature: 0.8,
                },
            },
            app: AppSettings {
                theme: "dark".to_string(),
            },
        }
    }
}

fn config_path() -> Result<PathBuf, AppError> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::Config("Could not find config directory".to_string()))?;
    let app_dir = config_dir.join("gm-practice-tool");
    fs::create_dir_all(&app_dir)?;
    Ok(app_dir.join("config.toml"))
}

pub fn load_config() -> Result<AppConfig, AppError> {
    let path = config_path()?;
    if !path.exists() {
        let config = AppConfig::default();
        save_config(&config)?;
        return Ok(config);
    }
    let content = fs::read_to_string(&path)?;
    let config: AppConfig = toml::from_str(&content)?;
    Ok(config)
}

pub fn save_config(config: &AppConfig) -> Result<(), AppError> {
    let path = config_path()?;
    let content = toml::to_string_pretty(config)?;
    fs::write(&path, content)?;
    Ok(())
}
