mod commands;
mod config;
mod db;
mod error;
mod llm;
mod models;

use commands::*;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async {
                let pool = db::create_pool().await.expect("Failed to create database pool");
                handle.manage(pool);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            update_config,
            create_ruleset,
            list_rulesets,
            get_ruleset,
            update_ruleset,
            delete_ruleset,
            create_campaign,
            list_campaigns,
            get_campaign,
            create_player,
            list_players,
            create_player_character,
            list_player_characters,
            get_messages,
            send_gm_message,
            get_campaign_log,
            get_player_memories,
            get_group_memories,
            delete_message,
            delete_player_memory,
            delete_group_memory,
            create_document,
            list_documents,
            update_document,
            delete_document,
            get_message_prompt,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
