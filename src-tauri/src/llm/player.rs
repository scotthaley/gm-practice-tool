use crate::config::AppConfig;
use crate::error::AppError;
use crate::llm::client::LlmClient;
use crate::llm::tools::{get_player_tools, roll_dice};
use crate::llm::types::*;
use crate::models::{CampaignLogEntry, GroupMemory, Player, PlayerMemory};
use serde_json::json;
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use uuid::Uuid;

pub struct PlayerContext {
    pub player: Player,
    pub memories: Vec<PlayerMemory>,
    pub group_memories: Vec<GroupMemory>,
    pub recent_log: Vec<CampaignLogEntry>,
}

pub async fn generate_player_response(
    client: &LlmClient,
    config: &AppConfig,
    pool: &SqlitePool,
    context: &PlayerContext,
    gm_message: &str,
    campaign_setting: &str,
) -> Result<(String, serde_json::Value), AppError> {
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

    let stats_str = serde_json::to_string_pretty(&context.player.stats).unwrap_or_default();

    let system = format!(
        r#"You are roleplaying as {name}, a Level {level} {race} {class} in a TTRPG campaign.

Setting: {setting}

Character Details:
- Name: {name}
- Race: {race}
- Class: {class}
- Level: {level}
- Personality: {personality}
- Backstory: {backstory}
- Stats: {stats}

Your Personal Memories:
{memories}

Shared Group Knowledge:
{group_memories}

Recent Events:
{recent_log}

Guidelines:
- Stay in character at all times
- Respond naturally to the GM's narration
- Use tools when appropriate (roll dice for checks, store important memories)
- Keep responses concise but flavorful (2-4 paragraphs max)
- React based on your character's personality and knowledge
- You may use the recall_memory tool if trying to remember something specific
- Use store_memory for significant new information your character would remember"#,
        name = context.player.name,
        race = context.player.race,
        class = context.player.class,
        level = context.player.level,
        personality = context.player.personality,
        backstory = context.player.backstory,
        stats = stats_str,
        setting = campaign_setting,
        memories = memories_text,
        group_memories = group_memories_text,
        recent_log = recent_log_text,
    );

    let mut messages = vec![ApiMessage {
        role: "user".to_string(),
        content: ApiContent::Text(format!("[GM]: {}", gm_message)),
    }];

    let tools = get_player_tools();
    let mut all_metadata = json!({ "tool_calls": [] });
    let mut final_text = String::new();

    // Tool use loop (max 5 iterations)
    for _ in 0..5 {
        let request = ApiRequest {
            model: config.models.player.clone(),
            max_tokens: config.models.parameters.player_max_tokens,
            temperature: Some(config.models.parameters.temperature),
            system: system.clone(),
            messages: messages.clone(),
            tools: Some(tools.clone()),
        };

        let response = client.send(&request).await?;

        let mut has_tool_use = false;
        let mut tool_results: Vec<ContentBlock> = Vec::new();
        let mut assistant_blocks: Vec<ContentBlock> = Vec::new();

        for block in &response.content {
            match block {
                ContentBlock::Text { text } => {
                    final_text = text.clone();
                    assistant_blocks.push(block.clone());
                }
                ContentBlock::ToolUse { id, name, input } => {
                    has_tool_use = true;
                    assistant_blocks.push(block.clone());

                    let result =
                        handle_tool_call(pool, &context.player, name, input).await?;

                    if let Some(calls) = all_metadata["tool_calls"].as_array_mut() {
                        calls.push(json!({
                            "tool": name,
                            "input": input,
                            "result": result
                        }));
                    }

                    tool_results.push(ContentBlock::ToolResult {
                        tool_use_id: id.clone(),
                        content: result,
                    });
                }
                _ => {}
            }
        }

        messages.push(ApiMessage {
            role: "assistant".to_string(),
            content: ApiContent::Blocks(assistant_blocks),
        });

        if !has_tool_use {
            break;
        }

        messages.push(ApiMessage {
            role: "user".to_string(),
            content: ApiContent::Blocks(tool_results),
        });
    }

    Ok((final_text, all_metadata))
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
                Ok(result) => Ok(format!(
                    "Rolling {} for {}: [{}] {} modifier = {} total",
                    result.notation,
                    reason,
                    result
                        .rolls
                        .iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    if result.modifier >= 0 { "+" } else { "" },
                    result.total
                )),
                Err(e) => Ok(format!("Failed to roll dice: {}", e)),
            }
        }
        "store_memory" => {
            let content = input["content"].as_str().unwrap_or("");
            let memory_type = input["memory_type"].as_str().unwrap_or("observation");
            let importance = input["importance"].as_f64().unwrap_or(0.5);
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
        _ => Ok(format!("Unknown tool: {}", tool_name)),
    }
}
