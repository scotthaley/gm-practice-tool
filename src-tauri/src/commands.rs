use crate::config::{self, AppConfig};
use crate::error::AppError;
use crate::llm::client::LlmClient;
use crate::llm::memory;
use crate::llm::player::{generate_player_response, PlayerContext};
use crate::llm::router::route_gm_message;
use crate::models::*;
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use tauri::State;
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

// Campaign commands
#[tauri::command]
pub async fn create_campaign(
    db: Db<'_>,
    request: CreateCampaignRequest,
) -> Result<Campaign, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO campaigns (id, name, description, setting, ruleset, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(&request.setting)
    .bind(&request.ruleset)
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
        ruleset: request.ruleset,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub async fn list_campaigns(db: Db<'_>) -> Result<Vec<Campaign>, AppError> {
    let rows = sqlx::query(
        "SELECT id, name, description, setting, ruleset, created_at, updated_at FROM campaigns ORDER BY updated_at DESC",
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
            ruleset: r.get("ruleset"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        })
        .collect())
}

#[tauri::command]
pub async fn get_campaign(db: Db<'_>, campaign_id: String) -> Result<Campaign, AppError> {
    let row = sqlx::query(
        "SELECT id, name, description, setting, ruleset, created_at, updated_at FROM campaigns WHERE id = ?",
    )
    .bind(&campaign_id)
    .fetch_optional(db.inner())
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
    .ok_or_else(|| AppError::Database("Campaign not found".to_string()))?;

    Ok(Campaign {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        setting: row.get("setting"),
        ruleset: row.get("ruleset"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

// Player commands
#[tauri::command]
pub async fn create_player(
    db: Db<'_>,
    request: CreatePlayerRequest,
) -> Result<Player, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let stats_json = serde_json::to_string(&request.stats).unwrap_or_default();

    sqlx::query(
        "INSERT INTO players (id, campaign_id, name, race, class, level, backstory, personality, stats, color, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&request.campaign_id)
    .bind(&request.name)
    .bind(&request.race)
    .bind(&request.class)
    .bind(request.level)
    .bind(&request.backstory)
    .bind(&request.personality)
    .bind(&stats_json)
    .bind(&request.color)
    .bind(&now)
    .execute(db.inner())
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Player {
        id,
        campaign_id: request.campaign_id,
        name: request.name,
        race: request.race,
        class: request.class,
        level: request.level,
        backstory: request.backstory,
        personality: request.personality,
        stats: request.stats,
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
        "SELECT id, campaign_id, name, race, class, level, backstory, personality, stats, color, created_at FROM players WHERE campaign_id = ? ORDER BY created_at",
    )
    .bind(campaign_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(rows
        .iter()
        .map(|r| {
            let stats_str: String = r.get("stats");
            Player {
                id: r.get("id"),
                campaign_id: r.get("campaign_id"),
                name: r.get("name"),
                race: r.get("race"),
                class: r.get("class"),
                level: r.get("level"),
                backstory: r.get("backstory"),
                personality: r.get("personality"),
                stats: serde_json::from_str(&stats_str).unwrap_or_default(),
                color: r.get("color"),
                created_at: r.get("created_at"),
            }
        })
        .collect())
}

pub async fn query_campaign(pool: &SqlitePool, campaign_id: &str) -> Result<Campaign, AppError> {
    let row = sqlx::query(
        "SELECT id, name, description, setting, ruleset, created_at, updated_at FROM campaigns WHERE id = ?",
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
        ruleset: row.get("ruleset"),
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

// The main GM message pipeline
#[tauri::command]
pub async fn send_gm_message(
    db: Db<'_>,
    campaign_id: String,
    message: String,
) -> Result<Vec<Message>, AppError> {
    let pool = db.inner();
    let config = config::load_config()?;
    let client = LlmClient::new(&config)?;
    let mut new_messages: Vec<Message> = Vec::new();

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

    new_messages.push(Message {
        id: gm_msg_id,
        campaign_id: campaign_id.clone(),
        sender_type: "gm".to_string(),
        sender_id: None,
        sender_name: "Game Master".to_string(),
        content: message.clone(),
        metadata: serde_json::json!({}),
        timestamp: now.clone(),
    });

    // Get campaign info
    let campaign = query_campaign(pool, &campaign_id).await?;

    // Get all players
    let players = query_players(pool, &campaign_id).await?;
    if players.is_empty() {
        return Ok(new_messages);
    }

    // Route the message
    let route_result =
        route_gm_message(&client, &config, &campaign.setting, &players, &message).await?;

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

        let player_memories =
            memory::get_player_memories(pool, &player.id, &campaign_id, 10).await?;
        let group_memories = memory::get_group_memories(pool, &campaign_id, 10).await?;
        let recent_log = memory::get_recent_log(pool, &campaign_id, 10).await?;

        let context = PlayerContext {
            player: player.clone(),
            memories: player_memories,
            group_memories,
            recent_log,
        };

        let (response_text, metadata) =
            generate_player_response(&client, &config, pool, &context, &message, &campaign.setting)
                .await?;

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

        new_messages.push(Message {
            id: msg_id,
            campaign_id: campaign_id.clone(),
            sender_type: "player".to_string(),
            sender_id: Some(player.id.clone()),
            sender_name: player.name.clone(),
            content: response_text,
            metadata,
            timestamp: msg_now,
        });
    }

    Ok(new_messages)
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
