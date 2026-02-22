use crate::error::AppError;
use crate::models::{CampaignLogEntry, GroupMemory, PlayerMemory};
use sqlx::sqlite::SqlitePool;
use sqlx::Row;

pub async fn get_player_memories(
    pool: &SqlitePool,
    player_id: &str,
    campaign_id: &str,
    limit: u32,
) -> Result<Vec<PlayerMemory>, AppError> {
    let rows = sqlx::query(
        "SELECT id, player_id, campaign_id, content, memory_type, importance, created_at FROM player_memory WHERE player_id = ? AND campaign_id = ? ORDER BY importance DESC, created_at DESC LIMIT ?",
    )
    .bind(player_id)
    .bind(campaign_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(rows
        .iter()
        .map(|r| PlayerMemory {
            id: r.get("id"),
            player_id: r.get("player_id"),
            campaign_id: r.get("campaign_id"),
            content: r.get("content"),
            memory_type: r.get("memory_type"),
            importance: r.get("importance"),
            created_at: r.get("created_at"),
        })
        .collect())
}

pub async fn get_group_memories(
    pool: &SqlitePool,
    campaign_id: &str,
    limit: u32,
) -> Result<Vec<GroupMemory>, AppError> {
    let rows = sqlx::query(
        "SELECT id, campaign_id, content, memory_type, importance, created_at FROM group_memory WHERE campaign_id = ? ORDER BY importance DESC, created_at DESC LIMIT ?",
    )
    .bind(campaign_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(rows
        .iter()
        .map(|r| GroupMemory {
            id: r.get("id"),
            campaign_id: r.get("campaign_id"),
            content: r.get("content"),
            memory_type: r.get("memory_type"),
            importance: r.get("importance"),
            created_at: r.get("created_at"),
        })
        .collect())
}

pub async fn get_recent_log(
    pool: &SqlitePool,
    campaign_id: &str,
    limit: u32,
) -> Result<Vec<CampaignLogEntry>, AppError> {
    let rows = sqlx::query(
        "SELECT id, campaign_id, entry_type, summary, details, timestamp FROM campaign_log WHERE campaign_id = ? ORDER BY timestamp DESC LIMIT ?",
    )
    .bind(campaign_id)
    .bind(limit)
    .fetch_all(pool)
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
