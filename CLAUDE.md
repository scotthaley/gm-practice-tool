# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

A Tauri v2 desktop app for practicing TTRPG game mastering. GMs type narrative input, and AI-powered player characters respond in-character using a two-stage LLM pipeline (router → player agents with tool use).

## Build & Development Commands

```bash
cargo tauri dev                                      # Full stack dev with hot reload
cargo check --manifest-path src-tauri/Cargo.toml     # Check Rust compilation
npx tsc --noEmit                                     # Check TypeScript
bun run build                                        # Build frontend only
bun install                                          # Install frontend dependencies
```

## Architecture

**Backend (Rust, `src-tauri/src/`):**
- `lib.rs` — App entry: SQLite pool setup in `setup()` hook, command registration
- `commands.rs` — All Tauri IPC command handlers; DB via `State<SqlitePool>`
- `db.rs` — SQLite pool creation and schema migrations (auto-run on startup)
- `models.rs` — Serde-annotated data models
- `config.rs` — TOML config loading from `dirs::config_dir()/gm-practice-tool/config.toml`
- `error.rs` — `AppError` enum using `thiserror`, auto-serialized for frontend

**LLM Pipeline (`src-tauri/src/llm/`):**
- `router.rs` — Haiku 4.5 classifies GM input → target players, log entry, group memories (temp 0.0)
- `player.rs` — Sonnet 4.6 generates in-character responses with tool use loop (max 5 iterations, temp 0.8)
- `tools.rs` — Tool definitions: `roll_dice`, `store_memory`, `add_campaign_log`, `recall_memory`
- `memory.rs` — DB queries for player/group/campaign memories (top 10 by importance)
- `client.rs` — Anthropic API HTTP client
- `types.rs` — API request/response type definitions

**Frontend (React/TypeScript, `src/`):**
- `stores/appStore.tsx` — Context + useReducer state management (campaigns, players, messages, view)
- `hooks/useChat.ts` — GM message sending and message management
- `hooks/useCampaign.ts` — Campaign loading and selection
- `lib/api.ts` — Typed wrappers around `@tauri-apps/api` invoke calls
- `lib/types.ts` — TypeScript interfaces matching Rust models

## Key Patterns

- **No frontend DB access** — All queries go through Rust commands via Tauri IPC
- **Adding a new command**: define in `commands.rs` → register in `lib.rs` → wrap in `lib/api.ts`
- **Adding a DB table**: add schema to `db.rs` → define model in `models.rs`
- **Adding a tool**: define in `tools.rs` → handle in `player.rs` tool call handler
- **Adding a view**: create component → add to `AppView` type → add routing in `App.tsx`

## Data Locations

- **Database**: `dirs::data_dir()/gm-practice-tool/gm_practice_tool.db`
- **Config**: `dirs::config_dir()/gm-practice-tool/config.toml` (see `config.example.toml`)

## GM Message Flow

1. Store GM message in DB
2. Router (Haiku) determines target players + generates log summary + group memories
3. For each target player: load context (memories, log) → Sonnet generates response with tool loop → store response
4. Return all new messages to frontend
