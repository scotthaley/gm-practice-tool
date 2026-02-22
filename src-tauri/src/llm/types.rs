use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest {
    pub model: String,
    pub max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    pub system: String,
    pub messages: Vec<ApiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMessage {
    pub role: String,
    pub content: ApiContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiContent {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub id: String,
    pub content: Vec<ContentBlock>,
    pub stop_reason: Option<String>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterResult {
    pub target_players: Vec<String>,
    pub log_summary: String,
    pub log_type: String,
    pub group_memories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub error: Option<ApiErrorDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorDetail {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}
