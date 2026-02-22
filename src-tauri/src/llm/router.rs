use crate::config::AppConfig;
use crate::error::AppError;
use crate::llm::client::LlmClient;
use crate::llm::types::{ApiContent, ApiMessage, ApiRequest, ContentBlock, RouterResult};
use crate::models::Player;

pub async fn route_gm_message(
    client: &LlmClient,
    config: &AppConfig,
    campaign_setting: &str,
    players: &[Player],
    gm_message: &str,
) -> Result<RouterResult, AppError> {
    let player_list = players
        .iter()
        .map(|p| format!("- {} ({}): {} {}, Level {}", p.name, p.id, p.race, p.class, p.level))
        .collect::<Vec<_>>()
        .join("\n");

    let system = format!(
        r#"You are a TTRPG routing assistant. Given a GM's message and the list of players, determine:
1. Which players should respond (by their IDs)
2. A brief log summary of what the GM described
3. The log entry type (one of: narrative, combat, social, exploration, description)
4. Any group memories worth storing (things all players would notice/know)

Campaign setting: {}

Players:
{}

Respond with ONLY valid JSON in this exact format:
{{
  "target_players": ["player-id-1", "player-id-2"],
  "log_summary": "Brief summary of GM's narration",
  "log_type": "narrative",
  "group_memories": ["Memory 1 if any"]
}}

If the GM is addressing all players or describing a scene, include all player IDs.
If addressing a specific player by name, only include that player."#,
        campaign_setting, player_list
    );

    let request = ApiRequest {
        model: config.models.router.clone(),
        max_tokens: config.models.parameters.router_max_tokens,
        temperature: Some(0.0),
        system,
        messages: vec![ApiMessage {
            role: "user".to_string(),
            content: ApiContent::Text(gm_message.to_string()),
        }],
        tools: None,
    };

    let response = client.send(&request).await?;

    let text = response
        .content
        .iter()
        .find_map(|block| match block {
            ContentBlock::Text { text } => Some(text.clone()),
            _ => None,
        })
        .unwrap_or_default();

    // Extract JSON from response (handle markdown code blocks)
    let json_str = if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            &text[start..=end]
        } else {
            &text
        }
    } else {
        &text
    };

    let result: RouterResult = serde_json::from_str(json_str).map_err(|e| {
        AppError::Llm(format!(
            "Failed to parse router response: {}. Response was: {}",
            e, text
        ))
    })?;

    Ok(result)
}
