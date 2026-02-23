use crate::error::AppError;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub async fn create_pool() -> Result<SqlitePool, AppError> {
    let data_dir = dirs::data_dir()
        .ok_or_else(|| AppError::Config("Could not find data directory".to_string()))?;
    let app_dir = data_dir.join("gm-practice-tool");
    std::fs::create_dir_all(&app_dir)?;
    let db_path = app_dir.join("gm_practice_tool.db");
    let url = format!("sqlite:{}?mode=rwc", db_path.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    run_migrations(&pool).await?;
    Ok(pool)
}

async fn run_migrations(pool: &SqlitePool) -> Result<(), AppError> {
    sqlx::raw_sql(
        "CREATE TABLE IF NOT EXISTS rulesets (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            content TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS campaigns (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            setting TEXT NOT NULL DEFAULT '',
            ruleset_id TEXT REFERENCES rulesets(id) ON DELETE SET NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS players (
            id TEXT PRIMARY KEY,
            campaign_id TEXT NOT NULL,
            name TEXT NOT NULL,
            personality TEXT NOT NULL DEFAULT '',
            color TEXT NOT NULL DEFAULT '#6366f1',
            created_at TEXT NOT NULL,
            FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS player_characters (
            id TEXT PRIMARY KEY,
            campaign_id TEXT NOT NULL,
            player_id TEXT,
            name TEXT NOT NULL,
            details TEXT NOT NULL DEFAULT '{}',
            created_at TEXT NOT NULL,
            FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
            FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE SET NULL
        );

        CREATE TABLE IF NOT EXISTS campaign_log (
            id TEXT PRIMARY KEY,
            campaign_id TEXT NOT NULL,
            entry_type TEXT NOT NULL DEFAULT 'narrative',
            summary TEXT NOT NULL,
            details TEXT NOT NULL DEFAULT '{}',
            timestamp TEXT NOT NULL,
            FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS player_memory (
            id TEXT PRIMARY KEY,
            player_id TEXT NOT NULL,
            campaign_id TEXT NOT NULL,
            content TEXT NOT NULL,
            memory_type TEXT NOT NULL DEFAULT 'observation',
            importance REAL NOT NULL DEFAULT 0.5,
            created_at TEXT NOT NULL,
            FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE,
            FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS group_memory (
            id TEXT PRIMARY KEY,
            campaign_id TEXT NOT NULL,
            content TEXT NOT NULL,
            memory_type TEXT NOT NULL DEFAULT 'event',
            importance REAL NOT NULL DEFAULT 0.5,
            created_at TEXT NOT NULL,
            FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            campaign_id TEXT NOT NULL,
            sender_type TEXT NOT NULL,
            sender_id TEXT,
            sender_name TEXT NOT NULL,
            content TEXT NOT NULL,
            metadata TEXT NOT NULL DEFAULT '{}',
            timestamp TEXT NOT NULL,
            FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_messages_campaign_timestamp ON messages(campaign_id, timestamp);
        CREATE INDEX IF NOT EXISTS idx_campaign_log_campaign_timestamp ON campaign_log(campaign_id, timestamp);
        CREATE INDEX IF NOT EXISTS idx_player_memory_player_importance ON player_memory(player_id, importance);
        CREATE INDEX IF NOT EXISTS idx_group_memory_campaign ON group_memory(campaign_id);
        CREATE INDEX IF NOT EXISTS idx_players_campaign ON players(campaign_id);
        CREATE INDEX IF NOT EXISTS idx_player_characters_player ON player_characters(player_id);
        CREATE INDEX IF NOT EXISTS idx_player_characters_campaign ON player_characters(campaign_id);

        CREATE TABLE IF NOT EXISTS documents (
            id TEXT PRIMARY KEY,
            campaign_id TEXT NOT NULL,
            name TEXT NOT NULL,
            content TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
            UNIQUE(campaign_id, name)
        );
        CREATE INDEX IF NOT EXISTS idx_documents_campaign ON documents(campaign_id);",
    )
    .execute(pool)
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    // Migrations (ignore errors for already-applied changes)
    let _ = sqlx::raw_sql(
        "ALTER TABLE player_characters ADD COLUMN pronouns TEXT NOT NULL DEFAULT '';",
    )
    .execute(pool)
    .await;

    Ok(())
}
