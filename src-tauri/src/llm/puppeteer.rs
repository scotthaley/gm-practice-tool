use crate::config::AppConfig;
use crate::llm::client::LlmClient;
use crate::llm::types::{ApiMessage, ApiRequest, PuppeteerResult};
use crate::models::{Message, Player, PlayerCharacter};

pub async fn determine_player_style(
    client: &LlmClient,
    config: &AppConfig,
    player: &Player,
    characters: &[PlayerCharacter],
    gm_message: &str,
    recent_messages: &[Message],
    log_type: &str,
) -> PuppeteerResult {
    let char_names: Vec<&str> = characters.iter().map(|c| c.name.as_str()).collect();
    let chars_desc = if char_names.is_empty() {
        "no character yet".to_string()
    } else {
        char_names.join(", ")
    };

    let recent_context: String = recent_messages
        .iter()
        .rev()
        .take(5)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|m| match m.sender_type.as_str() {
            "gm" => format!("[GM]: {}", m.content),
            _ => format!("[{}]: {}", m.sender_name, m.content),
        })
        .collect::<Vec<_>>()
        .join("\n");

    let system_prompt = format!(
        r#"You determine HOW a TTRPG player would stylistically respond to the GM. Given the player's personality and the scene, output ONLY JSON with these fields:
- hesitation_ms (0-3000): thinking pause in milliseconds. Higher for surprising/complex moments.
- casual_level ("low"/"medium"/"high"): speech formality. "high" = slang and casual, "low" = formal.
- brevity ("low"/"medium"/"high"): response length. "high" = very short/terse, "low" = longer/detailed.
- emotional_intensity ("restrained"/"moderate"/"intense"): how strongly emotions show.
- action_bias ("passive"/"balanced"/"proactive"): whether the character waits or takes initiative.

Player: {name} — {personality}
Characters: {chars}
Scene type: {log_type}"#,
        name = player.name,
        personality = player.personality,
        chars = chars_desc,
        log_type = log_type,
    );

    let mut messages = vec![ApiMessage {
        role: "system".to_string(),
        content: Some(system_prompt),
        tool_calls: None,
        tool_call_id: None,
    }];

    if !recent_context.is_empty() {
        messages.push(ApiMessage {
            role: "user".to_string(),
            content: Some(format!("Recent conversation:\n{}\n\nNew GM message: {}", recent_context, gm_message)),
            tool_calls: None,
            tool_call_id: None,
        });
    } else {
        messages.push(ApiMessage {
            role: "user".to_string(),
            content: Some(format!("GM message: {}", gm_message)),
            tool_calls: None,
            tool_call_id: None,
        });
    }

    let request = ApiRequest {
        model: config.models.router.clone(),
        max_tokens: 150,
        temperature: Some(0.0),
        messages,
        tools: None,
    };

    let response = match client.send(&request).await {
        Ok(r) => r,
        Err(_) => return PuppeteerResult::default(),
    };

    let text = response
        .choices
        .first()
        .and_then(|c| c.message.content.clone())
        .unwrap_or_default();

    // Extract JSON from response
    let json_str = if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            &text[start..=end]
        } else {
            return PuppeteerResult::default();
        }
    } else {
        return PuppeteerResult::default();
    };

    serde_json::from_str(json_str).unwrap_or_default()
}
