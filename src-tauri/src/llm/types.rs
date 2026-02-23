use serde::{Deserialize, Serialize};

// --- Request types (OpenAI-compatible for Groq) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest {
    pub model: String,
    pub max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    pub messages: Vec<ApiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinitionWrapper>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMessage {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: ToolCallFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

// --- Tool definitions (OpenAI format) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinitionWrapper {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ToolDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

// --- Response types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub id: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub message: ChoiceMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceMessage {
    pub role: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

// --- Router result (unchanged) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterResult {
    pub target_players: Vec<String>,
    pub log_summary: String,
    pub log_type: String,
    pub group_memories: Vec<String>,
    #[serde(default)]
    pub rule_updates: Vec<String>,
}

// --- Puppeteer result (style directives for player responses) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PuppeteerResult {
    pub hesitation_ms: u64,
    pub casual_level: String,
    pub brevity: String,
    pub emotional_intensity: String,
    pub action_bias: String,
}

impl Default for PuppeteerResult {
    fn default() -> Self {
        Self {
            hesitation_ms: 500,
            casual_level: "medium".to_string(),
            brevity: "medium".to_string(),
            emotional_intensity: "moderate".to_string(),
            action_bias: "balanced".to_string(),
        }
    }
}

// --- API error types (OpenAI-compatible) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub error: Option<ApiErrorDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub code: Option<String>,
}
