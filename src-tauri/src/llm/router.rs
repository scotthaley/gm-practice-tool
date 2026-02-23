use crate::config::AppConfig;
use crate::error::AppError;
use crate::llm::client::LlmClient;
use crate::llm::types::{ApiContent, ApiMessage, ApiRequest, ContentBlock, RouterResult};
use crate::models::{Player, PlayerCharacter};

fn summarize_character_details(details: &serde_json::Value) -> String {
    let mut parts = Vec::new();
    if let Some(race) = details.get("race").and_then(|v| v.as_str()) {
        if !race.is_empty() {
            parts.push(race.to_string());
        }
    }
    if let Some(class) = details.get("class").and_then(|v| v.as_str()) {
        if !class.is_empty() {
            parts.push(class.to_string());
        }
    }
    if let Some(level) = details.get("level").and_then(|v| v.as_i64()) {
        parts.push(format!("Level {}", level));
    }
    if parts.is_empty() {
        String::new()
    } else {
        parts.join(" ")
    }
}

pub async fn route_gm_message(
    client: &LlmClient,
    config: &AppConfig,
    campaign_setting: &str,
    ruleset_content: &str,
    players: &[Player],
    characters: &[PlayerCharacter],
    gm_message: &str,
) -> Result<RouterResult, AppError> {
    let mut entity_lines: Vec<String> = Vec::new();

    // List players and their characters
    for p in players {
        let player_chars: Vec<&PlayerCharacter> = characters
            .iter()
            .filter(|c| c.player_id.as_deref() == Some(&p.id))
            .collect();
        if player_chars.is_empty() {
            entity_lines.push(format!("- {} ({}): no character yet", p.name, p.id));
        } else {
            let char_descs: Vec<String> = player_chars
                .iter()
                .map(|c| {
                    let summary = summarize_character_details(&c.details);
                    if summary.is_empty() {
                        c.name.clone()
                    } else {
                        format!("{} ({})", c.name, summary)
                    }
                })
                .collect();
            entity_lines.push(format!(
                "- {} ({}): plays {}",
                p.name,
                p.id,
                char_descs.join(", ")
            ));
        }
    }

    // List NPC characters (no player assigned)
    let npcs: Vec<&PlayerCharacter> = characters
        .iter()
        .filter(|c| c.player_id.is_none())
        .collect();
    if !npcs.is_empty() {
        entity_lines.push("\nNPCs:".to_string());
        for c in &npcs {
            let summary = summarize_character_details(&c.details);
            if summary.is_empty() {
                entity_lines.push(format!("- {} (NPC)", c.name));
            } else {
                entity_lines.push(format!("- {} (NPC, {})", c.name, summary));
            }
        }
    }

    let player_list = entity_lines.join("\n");

    let system = format!(
        r#"You are a TTRPG routing assistant. Given a GM's message and the list of players, determine:
1. Which players should respond (by their IDs)
2. A brief log summary of what the GM described
3. The log entry type (one of: narrative, combat, social, exploration, description)
4. Any group memories worth storing (things all players would notice/know)

Campaign setting: {}
{}
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
        campaign_setting,
        if ruleset_content.is_empty() {
            String::new()
        } else {
            format!("\nRules:\n{}\n", ruleset_content)
        },
        player_list
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
