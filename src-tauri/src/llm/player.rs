use crate::config::AppConfig;
use crate::error::AppError;
use crate::llm::client::LlmClient;
use crate::llm::tools::{get_player_tools, roll_dice};
use crate::llm::types::*;
use crate::models::{CampaignLogEntry, GroupMemory, Message, Player, PlayerCharacter, PlayerMemory};
use serde_json::json;
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use uuid::Uuid;

fn deep_merge(base: &mut serde_json::Value, patch: &serde_json::Value) {
    match (base, patch) {
        (serde_json::Value::Object(base_map), serde_json::Value::Object(patch_map)) => {
            for (key, patch_value) in patch_map {
                let base_value = base_map
                    .entry(key.clone())
                    .or_insert(serde_json::Value::Null);
                deep_merge(base_value, patch_value);
            }
        }
        (base, patch) => {
            *base = patch.clone();
        }
    }
}

pub struct PlayerContext {
    pub player: Player,
    pub characters: Vec<PlayerCharacter>,
    pub memories: Vec<PlayerMemory>,
    pub group_memories: Vec<GroupMemory>,
    pub recent_log: Vec<CampaignLogEntry>,
}

fn format_character_details(characters: &[PlayerCharacter]) -> String {
    if characters.is_empty() {
        return "No characters assigned yet.".to_string();
    }

    characters
        .iter()
        .map(|c| {
            let mut lines = vec![format!("Character: {}", c.name)];
            if !c.pronouns.is_empty() {
                lines.push(format!("  Pronouns: {}", c.pronouns));
            }
            if let Some(obj) = c.details.as_object() {
                for (key, value) in obj {
                    let display = match value {
                        serde_json::Value::String(s) if !s.is_empty() => s.clone(),
                        serde_json::Value::Number(n) => n.to_string(),
                        serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
                            serde_json::to_string_pretty(value).unwrap_or_default()
                        }
                        _ => continue,
                    };
                    lines.push(format!("  {}: {}", key, display));
                }
            }
            lines.join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

pub async fn generate_player_response(
    client: &LlmClient,
    config: &AppConfig,
    pool: &SqlitePool,
    context: &PlayerContext,
    gm_message: &str,
    campaign_setting: &str,
    ruleset_content: &str,
    recent_messages: &[Message],
    style: &PuppeteerResult,
) -> Result<(String, serde_json::Value, String, String), AppError> {
    let memories_text = if context.memories.is_empty() {
        "No personal memories yet.".to_string()
    } else {
        context
            .memories
            .iter()
            .map(|m| format!("- [{}] {}", m.memory_type, m.content))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let group_memories_text = if context.group_memories.is_empty() {
        "No shared group memories yet.".to_string()
    } else {
        context
            .group_memories
            .iter()
            .map(|m| format!("- {}", m.content))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let recent_log_text = if context.recent_log.is_empty() {
        "No recent events.".to_string()
    } else {
        context
            .recent_log
            .iter()
            .map(|l| format!("- [{}] {}", l.entry_type, l.summary))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let character_details = format_character_details(&context.characters);

    let rules_section = if ruleset_content.is_empty() {
        String::new()
    } else {
        format!("\nRules:\n{}\n", ruleset_content)
    };

    let system_prompt = format!(
        r#"You are {name}, a player sitting at a TTRPG table. You are NOT a character — you are a person playing a game. Your character is a separate entity that you portray.

Setting: {setting}
{rules}
Your player personality (how you behave at the table, NOT your character's personality): {personality}

Your Characters (already created — use update_character to modify, do NOT re-create):
{character_details}

Your Personal Memories:
{memories}

Shared Group Knowledge:
{group_memories}

Recent Events:
{recent_log}

Response Style:
- Casualness: {casual_level} (low=formal, medium=natural, high=slang/casual)
- Brevity: {brevity} (low=detailed, medium=moderate, high=terse)
- Emotional intensity: {emotional_intensity} (restrained/moderate/intense)
- Action bias: {action_bias} (passive=wait for prompts, balanced=mix, proactive=take initiative)

Guidelines:
- Stay in character at all times
- Respond naturally to the GM's narration
- Use tools when appropriate (roll dice for checks, store important memories)
- Adjust response length based on your style directive above
- React based on your character's personality and knowledge
- You may use the recall_memory tool if trying to remember something specific
- Use store_memory for significant new information your character would remember
- If you already have characters listed above, use update_character to modify them — do NOT use create_character to duplicate an existing character
- Only use create_character when you have no characters yet or the GM asks you to create an additional new character
- Do ONLY what the GM asks. If the GM says to create a character with specific fields, include only those fields — do not add extra fields, and do not push the story forward unless the GM invites you to
- Before using store_memory, check whether the information is already present in "Your Personal Memories" or "Shared Group Knowledge" above. Do not store duplicate memories
- Your player personality describes how YOU behave at the table (e.g., cautious, goofy, strategic). Keep it separate from your character's traits — a goofy player can play a serious character
- Do NOT prefix your response with your name (e.g. "[{name}]:" or "{name}:") — just respond directly in character"#,
        name = context.player.name,
        personality = context.player.personality,
        character_details = character_details,
        setting = campaign_setting,
        rules = rules_section,
        memories = memories_text,
        group_memories = group_memories_text,
        recent_log = recent_log_text,
        casual_level = style.casual_level,
        brevity = style.brevity,
        emotional_intensity = style.emotional_intensity,
        action_bias = style.action_bias,
    );

    let mut messages = vec![ApiMessage {
        role: "system".to_string(),
        content: Some(system_prompt.clone()),
        tool_calls: None,
        tool_call_id: None,
    }];

    // Add recent conversation history for context
    // All history goes as "user" role to avoid teaching the model to prefix its name
    for msg in recent_messages {
        let content = match msg.sender_type.as_str() {
            "gm" => format!("[GM]: {}", msg.content),
            "player" => format!("[{}]: {}", msg.sender_name, msg.content),
            _ => continue,
        };
        messages.push(ApiMessage {
            role: "user".to_string(),
            content: Some(content),
            tool_calls: None,
            tool_call_id: None,
        });
    }

    // Add the current GM message
    messages.push(ApiMessage {
        role: "user".to_string(),
        content: Some(format!("[GM]: {}", gm_message)),
        tool_calls: None,
        tool_call_id: None,
    });

    let tools = get_player_tools();
    let mut all_metadata = json!({ "tool_calls": [] });
    let mut final_text = String::new();

    // Capture prompt data before the tool loop
    let prompt_data = serde_json::to_string(&messages).unwrap_or_default();
    let mut all_responses: Vec<serde_json::Value> = Vec::new();

    // Tool use loop (max 5 iterations)
    for _ in 0..5 {
        let request = ApiRequest {
            model: config.models.player.clone(),
            max_tokens: config.models.parameters.player_max_tokens,
            temperature: Some(config.models.parameters.temperature),
            messages: messages.clone(),
            tools: Some(tools.clone()),
        };

        let response = client.send(&request).await?;
        all_responses.push(serde_json::to_value(&response).unwrap_or_default());

        let choice = response
            .choices
            .first()
            .ok_or_else(|| AppError::Llm("No choices in API response".to_string()))?;

        // Extract text content if present
        if let Some(text) = &choice.message.content {
            if !text.is_empty() {
                final_text = text.clone();
            }
        }

        let tool_calls = choice.message.tool_calls.clone().unwrap_or_default();

        // Add the assistant message to history (required by OpenAI format)
        messages.push(ApiMessage {
            role: "assistant".to_string(),
            content: choice.message.content.clone(),
            tool_calls: if tool_calls.is_empty() {
                None
            } else {
                Some(tool_calls.clone())
            },
            tool_call_id: None,
        });

        if tool_calls.is_empty() {
            break;
        }

        // Process each tool call and add results
        for tc in &tool_calls {
            let input: serde_json::Value =
                serde_json::from_str(&tc.function.arguments).unwrap_or(json!({}));

            let result = handle_tool_call(
                pool,
                &context.player,
                &tc.function.name,
                &input,
            )
            .await?;

            if let Some(calls) = all_metadata["tool_calls"].as_array_mut() {
                calls.push(json!({
                    "tool": tc.function.name,
                    "input": input,
                    "result": result
                }));
            }

            // Tool results use role "tool" with tool_call_id
            messages.push(ApiMessage {
                role: "tool".to_string(),
                content: Some(result),
                tool_calls: None,
                tool_call_id: Some(tc.id.clone()),
            });
        }
    }

    let llm_response = serde_json::to_string(&all_responses).unwrap_or_default();

    Ok((final_text, all_metadata, prompt_data, llm_response))
}

async fn handle_tool_call(
    pool: &SqlitePool,
    player: &Player,
    tool_name: &str,
    input: &serde_json::Value,
) -> Result<String, AppError> {
    match tool_name {
        "roll_dice" => {
            let notation = input["notation"].as_str().unwrap_or("1d20");
            let reason = input["reason"].as_str().unwrap_or("unknown");
            match roll_dice(notation) {
                Ok(result) => {
                    let rolls_str = result
                        .rolls
                        .iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    if result.modifier != 0 {
                        Ok(format!(
                            "Rolling {} for {}: [{}] {} {} = {}",
                            result.notation,
                            reason,
                            rolls_str,
                            if result.modifier >= 0 { "+" } else { "-" },
                            result.modifier.unsigned_abs(),
                            result.total
                        ))
                    } else {
                        Ok(format!(
                            "Rolling {} for {}: [{}] = {}",
                            result.notation,
                            reason,
                            rolls_str,
                            result.total
                        ))
                    }
                }
                Err(e) => Ok(format!("Failed to roll dice: {}", e)),
            }
        }
        "store_memory" => {
            let content = input["content"].as_str().unwrap_or("");
            let memory_type = input["memory_type"].as_str().unwrap_or("observation");
            let importance = match input["importance"].as_str().unwrap_or("medium") {
                "low" => 0.25,
                "medium" => 0.5,
                "high" => 0.75,
                "critical" => 1.0,
                _ => 0.5,
            };
            let id = Uuid::new_v4().to_string();
            let now = chrono::Utc::now().to_rfc3339();

            sqlx::query(
                "INSERT INTO player_memory (id, player_id, campaign_id, content, memory_type, importance, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&id)
            .bind(&player.id)
            .bind(&player.campaign_id)
            .bind(content)
            .bind(memory_type)
            .bind(importance)
            .bind(&now)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

            Ok(format!("Memory stored: {}", content))
        }
        "add_campaign_log" => {
            let summary = input["summary"].as_str().unwrap_or("");
            let entry_type = input["entry_type"].as_str().unwrap_or("action");
            let id = Uuid::new_v4().to_string();
            let now = chrono::Utc::now().to_rfc3339();

            sqlx::query(
                "INSERT INTO campaign_log (id, campaign_id, entry_type, summary, details, timestamp) VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(&id)
            .bind(&player.campaign_id)
            .bind(entry_type)
            .bind(summary)
            .bind("{}")
            .bind(&now)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

            Ok(format!("Log entry added: {}", summary))
        }
        "recall_memory" => {
            let query = input["query"].as_str().unwrap_or("");
            let rows = sqlx::query(
                "SELECT id, player_id, campaign_id, content, memory_type, importance, created_at FROM player_memory WHERE player_id = ? AND content LIKE ? ORDER BY importance DESC LIMIT 5",
            )
            .bind(&player.id)
            .bind(format!("%{}%", query))
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

            if rows.is_empty() {
                Ok(format!("No memories found related to '{}'", query))
            } else {
                let text = rows
                    .iter()
                    .map(|r| {
                        let mt: String = r.get("memory_type");
                        let content: String = r.get("content");
                        format!("- [{}] {}", mt, content)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                Ok(format!("Recalled memories:\n{}", text))
            }
        }
        "create_character" => {
            let name = input["name"].as_str().unwrap_or("Unnamed");
            let pronouns = input["pronouns"].as_str().unwrap_or("");
            let details = input.get("details").cloned().unwrap_or(json!({}));
            let id = Uuid::new_v4().to_string();
            let now = chrono::Utc::now().to_rfc3339();

            sqlx::query(
                "INSERT INTO player_characters (id, campaign_id, player_id, name, pronouns, details, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&id)
            .bind(&player.campaign_id)
            .bind(&player.id)
            .bind(name)
            .bind(pronouns)
            .bind(serde_json::to_string(&details).unwrap_or_default())
            .bind(&now)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

            Ok(format!("Character '{}' created successfully.", name))
        }
        "update_character" => {
            let char_name = input["character_name"].as_str().unwrap_or("");
            let updates = input.get("updates").cloned().unwrap_or(json!({}));
            let new_pronouns = input["pronouns"].as_str();

            // Look up character by name and player_id
            let row = sqlx::query(
                "SELECT id, details, pronouns FROM player_characters WHERE name = ? AND player_id = ?",
            )
            .bind(char_name)
            .bind(&player.id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

            match row {
                Some(row) => {
                    let char_id: String = row.get("id");
                    let existing_details_str: String = row.get("details");
                    let mut existing_details: serde_json::Value =
                        serde_json::from_str(&existing_details_str).unwrap_or(json!({}));

                    // Deep merge updates into existing details
                    deep_merge(&mut existing_details, &updates);

                    let existing_pronouns: String = row.get("pronouns");
                    let pronouns_val = new_pronouns.unwrap_or(&existing_pronouns);

                    sqlx::query(
                        "UPDATE player_characters SET details = ?, pronouns = ? WHERE id = ?",
                    )
                    .bind(serde_json::to_string(&existing_details).unwrap_or_default())
                    .bind(pronouns_val)
                    .bind(&char_id)
                    .execute(pool)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?;

                    let updated_keys: Vec<String> = updates
                        .as_object()
                        .map(|o| o.keys().cloned().collect())
                        .unwrap_or_default();
                    Ok(format!(
                        "Character '{}' updated: {}",
                        char_name,
                        updated_keys.join(", ")
                    ))
                }
                None => Ok(format!("No character named '{}' found for this player.", char_name)),
            }
        }
        _ => Ok(format!("Unknown tool: {}", tool_name)),
    }
}
