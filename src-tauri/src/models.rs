use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ruleset {
    pub id: String,
    pub name: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRulesetRequest {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRulesetRequest {
    pub id: String,
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub id: String,
    pub name: String,
    pub description: String,
    pub setting: String,
    pub ruleset_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub campaign_id: String,
    pub name: String,
    pub personality: String,
    pub color: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerCharacter {
    pub id: String,
    pub campaign_id: String,
    pub player_id: Option<String>,
    pub name: String,
    pub pronouns: String,
    pub details: serde_json::Value,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignLogEntry {
    pub id: String,
    pub campaign_id: String,
    pub entry_type: String,
    pub summary: String,
    pub details: serde_json::Value,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMemory {
    pub id: String,
    pub player_id: String,
    pub campaign_id: String,
    pub content: String,
    pub memory_type: String,
    pub importance: f64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMemory {
    pub id: String,
    pub campaign_id: String,
    pub content: String,
    pub memory_type: String,
    pub importance: f64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub campaign_id: String,
    pub sender_type: String,
    pub sender_id: Option<String>,
    pub sender_name: String,
    pub content: String,
    pub metadata: serde_json::Value,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub description: String,
    pub setting: String,
    pub ruleset_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlayerRequest {
    pub campaign_id: String,
    pub name: String,
    pub personality: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlayerCharacterRequest {
    pub campaign_id: String,
    pub player_id: Option<String>,
    pub name: String,
    pub pronouns: String,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub campaign_id: String,
    pub name: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePlayerCharacterRequest {
    pub id: String,
    pub name: String,
    pub pronouns: String,
    pub player_id: Option<String>,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentRequest {
    pub campaign_id: String,
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentRequest {
    pub id: String,
    pub name: String,
    pub content: String,
}

// Event payloads for real-time UI updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTypingEvent {
    pub campaign_id: String,
    pub player_id: String,
    pub player_name: String,
    pub player_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEvent {
    pub message: Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationCompleteEvent {
    pub campaign_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationErrorEvent {
    pub campaign_id: String,
    pub error: String,
}
