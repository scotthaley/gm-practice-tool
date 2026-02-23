use crate::config::{self, AppConfig};
use crate::error::AppError;
use crate::llm::client::LlmClient;
use crate::llm::memory;
use crate::llm::player::{generate_player_response, PlayerContext};
use crate::llm::router::route_gm_message;
use crate::models::*;
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use tauri::{Emitter, State};
use uuid::Uuid;

type Db<'a> = State<'a, SqlitePool>;

// Config commands
#[tauri::command]
pub async fn get_config() -> Result<AppConfig, AppError> {
    config::load_config()
}

#[tauri::command]
pub async fn update_config(config: AppConfig) -> Result<(), AppError> {
    config::save_config(&config)
}

// Ruleset commands
#[tauri::command]
pub async fn create_ruleset(
    db: Db<'_>,
    request: CreateRulesetRequest,
) -> Result<Ruleset, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO rulesets (id, name, content, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&request.name)
    .bind(&request.content)
    .bind(&now)
    .bind(&now)
    .execute(db.inner())
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Ruleset {
        id,
        name: request.name,
        content: request.content,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub async fn list_rulesets(db: Db<'_>) -> Result<Vec<Ruleset>, AppError> {
    let rows = sqlx::query(
        "SELECT id, name, content, created_at, updated_at FROM rulesets ORDER BY updated_at DESC",
    )
    .fetch_all(db.inner())
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(rows
        .iter()
        .map(|r| Ruleset {
            id: r.get("id"),
            name: r.get("name"),
            content: r.get("content"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        })
        .collect())
}

#[tauri::command]
pub async fn get_ruleset(db: Db<'_>, ruleset_id: String) -> Result<Ruleset, AppError> {
    query_ruleset(db.inner(), &ruleset_id).await
}

#[tauri::command]
pub async fn update_ruleset(
    db: Db<'_>,
    request: UpdateRulesetRequest,
) -> Result<Ruleset, AppError> {
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query("UPDATE rulesets SET name = ?, content = ?, updated_at = ? WHERE id = ?")
        .bind(&request.name)
        .bind(&request.content)
        .bind(&now)
        .bind(&request.id)
        .execute(db.inner())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    query_ruleset(db.inner(), &request.id).await
}

#[tauri::command]
pub async fn delete_ruleset(db: Db<'_>, ruleset_id: String) -> Result<(), AppError> {
    sqlx::query("DELETE FROM rulesets WHERE id = ?")
        .bind(&ruleset_id)
        .execute(db.inner())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(())
}

pub async fn query_ruleset(pool: &SqlitePool, ruleset_id: &str) -> Result<Ruleset, AppError> {
    let row = sqlx::query(
        "SELECT id, name, content, created_at, updated_at FROM rulesets WHERE id = ?",
    )
    .bind(ruleset_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
    .ok_or_else(|| AppError::Database("Ruleset not found".to_string()))?;

    Ok(Ruleset {
        id: row.get("id"),
        name: row.get("name"),
        content: row.get("content"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

// Campaign commands
#[tauri::command]
pub async fn create_campaign(
    db: Db<'_>,
    request: CreateCampaignRequest,
) -> Result<Campaign, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO campaigns (id, name, description, setting, ruleset_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.setting)
    .bind(&request.ruleset_id)
    .bind(&now)
    .bind(&now)
    .execute(db.inner())
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Campaign {
        id,
        name: request.name,
        description: request.description,
        setting: request.setting,
        ruleset_id: request.ruleset_id,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub async fn list_campaigns(db: Db<'_>) -> Result<Vec<Campaign>, AppError> {
    let rows = sqlx::query(
        "SELECT id, name, description, setting, ruleset_id, created_at, updated_at FROM campaigns ORDER BY updated_at DESC",
    )
    .fetch_all(db.inner())
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(rows
        .iter()
        .map(|r| Campaign {
            id: r.get("id"),
            name: r.get("name"),
            description: r.get("description"),
            setting: r.get("setting"),
            ruleset_id: r.get("ruleset_id"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        })
        .collect())
}

#[tauri::command]
pub async fn get_campaign(db: Db<'_>, campaign_id: String) -> Result<Campaign, AppError> {
    query_campaign(db.inner(), &campaign_id).await
}

// Player commands
#[tauri::command]
pub async fn create_player(
    db: Db<'_>,
    request: CreatePlayerRequest,
) -> Result<Player, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO players (id, campaign_id, name, personality, color, created_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&request.campaign_id)
    .bind(&request.name)
    .bind(&request.personality)
    .bind(&request.color)
    .bind(&now)
    .execute(db.inner())
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Player {
        id,
        campaign_id: request.campaign_id,
        name: request.name,
        personality: request.personality,
        color: request.color,
        created_at: now,
    })
}

#[tauri::command]
pub async fn list_players(
    db: Db<'_>,
    campaign_id: String,
) -> Result<Vec<Player>, AppError> {
    query_players(db.inner(), &campaign_id).await
}

pub async fn query_players(pool: &SqlitePool, campaign_id: &str) -> Result<Vec<Player>, AppError> {
    let rows = sqlx::query(
        "SELECT id, campaign_id, name, personality, color, created_at FROM players WHERE campaign_id = ? ORDER BY created_at",
    )
    .bind(campaign_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(rows
        .iter()
        .map(|r| Player {
            id: r.get("id"),
            campaign_id: r.get("campaign_id"),
            name: r.get("name"),
            personality: r.get("personality"),
            color: r.get("color"),
            created_at: r.get("created_at"),
        })
        .collect())
}

// Player character commands
#[tauri::command]
pub async fn create_player_character(
    db: Db<'_>,
    request: CreatePlayerCharacterRequest,
) -> Result<PlayerCharacter, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let details_json = serde_json::to_string(&request.details).unwrap_or_default();

    sqlx::query(
        "INSERT INTO player_characters (id, campaign_id, player_id, name, pronouns, details, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&request.campaign_id)
    .bind(&request.player_id)
    .bind(&request.name)
    .bind(&request.pronouns)
    .bind(&details_json)
    .bind(&now)
    .execute(db.inner())
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(PlayerCharacter {
        id,
        campaign_id: request.campaign_id,
        player_id: request.player_id,
        name: request.name,
        pronouns: request.pronouns,
        details: request.details,
        created_at: now,
    })
}

#[tauri::command]
pub async fn list_player_characters(
    db: Db<'_>,
    campaign_id: String,
) -> Result<Vec<PlayerCharacter>, AppError> {
    query_player_characters(db.inner(), &campaign_id).await
}

pub async fn query_player_characters(
    pool: &SqlitePool,
    campaign_id: &str,
) -> Result<Vec<PlayerCharacter>, AppError> {
    let rows = sqlx::query(
        "SELECT id, campaign_id, player_id, name, pronouns, details, created_at FROM player_characters WHERE campaign_id = ? ORDER BY created_at",
    )
    .bind(campaign_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(rows
        .iter()
        .map(|r| {
            let details_str: String = r.get("details");
            PlayerCharacter {
                id: r.get("id"),
                campaign_id: r.get("campaign_id"),
                player_id: r.get("player_id"),
                name: r.get("name"),
                pronouns: r.get("pronouns"),
                details: serde_json::from_str(&details_str).unwrap_or_default(),
                created_at: r.get("created_at"),
            }
        })
        .collect())
}

pub async fn query_campaign(pool: &SqlitePool, campaign_id: &str) -> Result<Campaign, AppError> {
    let row = sqlx::query(
        "SELECT id, name, description, setting, ruleset_id, created_at, updated_at FROM campaigns WHERE id = ?",
    )
    .bind(campaign_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
    .ok_or_else(|| AppError::Database("Campaign not found".to_string()))?;

    Ok(Campaign {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        setting: row.get("setting"),
        ruleset_id: row.get("ruleset_id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

// Message commands
#[tauri::command]
pub async fn get_messages(
    db: Db<'_>,
    campaign_id: String,
) -> Result<Vec<Message>, AppError> {
    query_messages(db.inner(), &campaign_id).await
}

pub async fn query_messages(pool: &SqlitePool, campaign_id: &str) -> Result<Vec<Message>, AppError> {
    let rows = sqlx::query(
        "SELECT id, campaign_id, sender_type, sender_id, sender_name, content, metadata, timestamp FROM messages WHERE campaign_id = ? ORDER BY timestamp ASC",
    )
    .bind(campaign_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(rows
        .iter()
        .map(|r| {
            let metadata_str: String = r.get("metadata");
            Message {
                id: r.get("id"),
                campaign_id: r.get("campaign_id"),
                sender_type: r.get("sender_type"),
                sender_id: r.get("sender_id"),
                sender_name: r.get("sender_name"),
                content: r.get("content"),
                metadata: serde_json::from_str(&metadata_str).unwrap_or_default(),
                timestamp: r.get("timestamp"),
            }
        })
        .collect())
}

// Document commands
#[tauri::command]
pub async fn create_document(
    db: Db<'_>,
    request: CreateDocumentRequest,
) -> Result<Document, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO documents (id, campaign_id, name, content, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&request.campaign_id)
    .bind(&request.name)
    .bind(&request.content)
    .bind(&now)
    .bind(&now)
    .execute(db.inner())
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Document {
        id,
        campaign_id: request.campaign_id,
        name: request.name,
        content: request.content,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub async fn list_documents(
    db: Db<'_>,
    campaign_id: String,
) -> Result<Vec<Document>, AppError> {
    let rows = sqlx::query(
        "SELECT id, campaign_id, name, content, created_at, updated_at FROM documents WHERE campaign_id = ? ORDER BY name",
    )
    .bind(&campaign_id)
    .fetch_all(db.inner())
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(rows
        .iter()
        .map(|r| Document {
            id: r.get("id"),
            campaign_id: r.get("campaign_id"),
            name: r.get("name"),
            content: r.get("content"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        })
        .collect())
}

#[tauri::command]
pub async fn update_document(
    db: Db<'_>,
    request: UpdateDocumentRequest,
) -> Result<Document, AppError> {
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query("UPDATE documents SET name = ?, content = ?, updated_at = ? WHERE id = ?")
        .bind(&request.name)
        .bind(&request.content)
        .bind(&now)
        .bind(&request.id)
        .execute(db.inner())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let row = sqlx::query(
        "SELECT id, campaign_id, name, content, created_at, updated_at FROM documents WHERE id = ?",
    )
    .bind(&request.id)
    .fetch_one(db.inner())
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Document {
        id: row.get("id"),
        campaign_id: row.get("campaign_id"),
        name: row.get("name"),
        content: row.get("content"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

#[tauri::command]
pub async fn delete_document(db: Db<'_>, document_id: String) -> Result<(), AppError> {
    sqlx::query("DELETE FROM documents WHERE id = ?")
        .bind(&document_id)
        .execute(db.inner())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(())
}

// The main GM message pipeline
#[tauri::command]
pub async fn send_gm_message(
    db: Db<'_>,
    app_handle: tauri::AppHandle,
    campaign_id: String,
    message: String,
) -> Result<(), AppError> {
    let pool = db.inner();
    let config = config::load_config()?;
    let client = LlmClient::new(&config)?;

    // Store GM message
    let gm_msg_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO messages (id, campaign_id, sender_type, sender_id, sender_name, content, metadata, timestamp) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&gm_msg_id)
    .bind(&campaign_id)
    .bind("gm")
    .bind(Option::<String>::None)
    .bind("Game Master")
    .bind(&message)
    .bind("{}")
    .bind(&now)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let gm_message = Message {
        id: gm_msg_id,
        campaign_id: campaign_id.clone(),
        sender_type: "gm".to_string(),
        sender_id: None,
        sender_name: "Game Master".to_string(),
        content: message.clone(),
        metadata: serde_json::json!({}),
        timestamp: now.clone(),
    };
    let _ = app_handle.emit("gm:message_stored", MessageEvent { message: gm_message });

    // Resolve @DocName references for LLM
    let documents = {
        let rows = sqlx::query(
            "SELECT name, content FROM documents WHERE campaign_id = ? ORDER BY LENGTH(name) DESC",
        )
        .bind(&campaign_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        rows.iter()
            .map(|r| (r.get::<String, _>("name"), r.get::<String, _>("content")))
            .collect::<Vec<(String, String)>>()
    };

    let mut resolved_message = message.clone();
    for (name, content) in &documents {
        let tag = format!("@{}", name);
        if resolved_message.contains(&tag) {
            let replacement = format!("[Document: {}]\n{}\n[/Document]", name, content);
            resolved_message = resolved_message.replace(&tag, &replacement);
        }
    }

    // Get campaign info
    let campaign = query_campaign(pool, &campaign_id).await?;

    // Fetch ruleset content if campaign has one
    let ruleset_content = if let Some(ref rid) = campaign.ruleset_id {
        match query_ruleset(pool, rid).await {
            Ok(rs) => rs.content,
            Err(_) => String::new(),
        }
    } else {
        String::new()
    };

    // Get all players and characters
    let players = query_players(pool, &campaign_id).await?;
    let characters = query_player_characters(pool, &campaign_id).await?;
    if players.is_empty() {
        let _ = app_handle.emit("gm:generation_complete", GenerationCompleteEvent { campaign_id });
        return Ok(());
    }

    // Load recent messages for routing context
    let context_message_count = config.app.context_messages;
    let routing_messages: Vec<Message> = if context_message_count > 0 {
        load_recent_messages(pool, &campaign_id, context_message_count).await?
    } else {
        Vec::new()
    };

    // Route the message (using resolved text with document content)
    let route_result = route_gm_message(
        &client,
        &config,
        &campaign.setting,
        &ruleset_content,
        &players,
        &characters,
        &resolved_message,
        &routing_messages,
    )
    .await?;

    // Store campaign log entry
    let log_id = Uuid::new_v4().to_string();
    let log_now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO campaign_log (id, campaign_id, entry_type, summary, details, timestamp) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&log_id)
    .bind(&campaign_id)
    .bind(&route_result.log_type)
    .bind(&route_result.log_summary)
    .bind("{}")
    .bind(&log_now)
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Store group memories
    for mem_content in &route_result.group_memories {
        if !mem_content.is_empty() {
            let mem_id = Uuid::new_v4().to_string();
            let mem_now = chrono::Utc::now().to_rfc3339();
            sqlx::query(
                "INSERT INTO group_memory (id, campaign_id, content, memory_type, importance, created_at) VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(&mem_id)
            .bind(&campaign_id)
            .bind(mem_content)
            .bind("event")
            .bind(0.6f64)
            .bind(&mem_now)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        }
    }

    // Generate response for each target player
    for player_id in &route_result.target_players {
        let player = match players.iter().find(|p| &p.id == player_id) {
            Some(p) => p.clone(),
            None => continue,
        };

        // Emit typing indicator
        let _ = app_handle.emit("gm:player_typing", PlayerTypingEvent {
            campaign_id: campaign_id.clone(),
            player_id: player.id.clone(),
            player_name: player.name.clone(),
            player_color: player.color.clone(),
        });

        let player_characters: Vec<PlayerCharacter> = characters
            .iter()
            .filter(|c| c.player_id.as_deref() == Some(&player.id))
            .cloned()
            .collect();

        let player_memories =
            memory::get_player_memories(pool, &player.id, &campaign_id, 10).await?;
        let group_memories = memory::get_group_memories(pool, &campaign_id, 10).await?;
        let recent_log = memory::get_recent_log(pool, &campaign_id, 10).await?;

        let context = PlayerContext {
            player: player.clone(),
            characters: player_characters,
            memories: player_memories,
            group_memories,
            recent_log,
            ruleset_id: campaign.ruleset_id.clone(),
        };

        // Load fresh recent messages so each player sees previous players' responses
        let recent_messages: Vec<Message> = if context_message_count > 0 {
            load_recent_messages(pool, &campaign_id, context_message_count).await?
        } else {
            Vec::new()
        };

        let result = generate_player_response(
            &client,
            &config,
            pool,
            &context,
            &resolved_message,
            &campaign.setting,
            &ruleset_content,
            &recent_messages,
        )
        .await;

        match result {
            Ok((response_text, metadata)) => {
                // Store player message
                let msg_id = Uuid::new_v4().to_string();
                let msg_now = chrono::Utc::now().to_rfc3339();
                let metadata_str = serde_json::to_string(&metadata).unwrap_or_default();
                sqlx::query(
                    "INSERT INTO messages (id, campaign_id, sender_type, sender_id, sender_name, content, metadata, timestamp) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                )
                .bind(&msg_id)
                .bind(&campaign_id)
                .bind("player")
                .bind(&player.id)
                .bind(&player.name)
                .bind(&response_text)
                .bind(&metadata_str)
                .bind(&msg_now)
                .execute(pool)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

                let player_message = Message {
                    id: msg_id,
                    campaign_id: campaign_id.clone(),
                    sender_type: "player".to_string(),
                    sender_id: Some(player.id.clone()),
                    sender_name: player.name.clone(),
                    content: response_text,
                    metadata,
                    timestamp: msg_now,
                };
                let _ = app_handle.emit("gm:player_response", MessageEvent { message: player_message });
            }
            Err(e) => {
                let _ = app_handle.emit("gm:generation_error", GenerationErrorEvent {
                    campaign_id: campaign_id.clone(),
                    error: format!("{} failed: {}", player.name, e),
                });
                continue;
            }
        }
    }

    let _ = app_handle.emit("gm:generation_complete", GenerationCompleteEvent { campaign_id });
    Ok(())
}

async fn load_recent_messages(
    pool: &SqlitePool,
    campaign_id: &str,
    limit: u32,
) -> Result<Vec<Message>, AppError> {
    let rows = sqlx::query(
        "SELECT id, campaign_id, sender_type, sender_id, sender_name, content, metadata, timestamp FROM messages WHERE campaign_id = ? ORDER BY timestamp DESC LIMIT ?",
    )
    .bind(campaign_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    let mut msgs: Vec<Message> = rows
        .iter()
        .map(|r| {
            let metadata_str: String = r.get("metadata");
            Message {
                id: r.get("id"),
                campaign_id: r.get("campaign_id"),
                sender_type: r.get("sender_type"),
                sender_id: r.get("sender_id"),
                sender_name: r.get("sender_name"),
                content: r.get("content"),
                metadata: serde_json::from_str(&metadata_str).unwrap_or_default(),
                timestamp: r.get("timestamp"),
            }
        })
        .collect();
    msgs.reverse(); // Chronological order
    Ok(msgs)
}

// Delete commands
#[tauri::command]
pub async fn delete_message(db: Db<'_>, message_id: String) -> Result<(), AppError> {
    sqlx::query("DELETE FROM messages WHERE id = ?")
        .bind(&message_id)
        .execute(db.inner())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub async fn delete_player_memory(db: Db<'_>, memory_id: String) -> Result<(), AppError> {
    sqlx::query("DELETE FROM player_memory WHERE id = ?")
        .bind(&memory_id)
        .execute(db.inner())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub async fn delete_group_memory(db: Db<'_>, memory_id: String) -> Result<(), AppError> {
    sqlx::query("DELETE FROM group_memory WHERE id = ?")
        .bind(&memory_id)
        .execute(db.inner())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(())
}

// Player memory commands
#[tauri::command]
pub async fn get_player_memories(
    db: Db<'_>,
    player_id: String,
    campaign_id: String,
) -> Result<Vec<PlayerMemory>, AppError> {
    memory::get_player_memories(db.inner(), &player_id, &campaign_id, 50).await
}

#[tauri::command]
pub async fn get_group_memories(
    db: Db<'_>,
    campaign_id: String,
) -> Result<Vec<GroupMemory>, AppError> {
    memory::get_group_memories(db.inner(), &campaign_id, 50).await
}

// Campaign log commands
#[tauri::command]
pub async fn get_campaign_log(
    db: Db<'_>,
    campaign_id: String,
) -> Result<Vec<CampaignLogEntry>, AppError> {
    let rows = sqlx::query(
        "SELECT id, campaign_id, entry_type, summary, details, timestamp FROM campaign_log WHERE campaign_id = ? ORDER BY timestamp DESC",
    )
    .bind(&campaign_id)
    .fetch_all(db.inner())
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(rows
        .iter()
        .map(|r| {
            let details_str: String = r.get("details");
            CampaignLogEntry {
                id: r.get("id"),
                campaign_id: r.get("campaign_id"),
                entry_type: r.get("entry_type"),
                summary: r.get("summary"),
                details: serde_json::from_str(&details_str).unwrap_or_default(),
                timestamp: r.get("timestamp"),
            }
        })
        .collect())
}
