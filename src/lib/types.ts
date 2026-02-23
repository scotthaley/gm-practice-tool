export interface Ruleset {
  id: string;
  name: string;
  content: string;
  created_at: string;
  updated_at: string;
}

export interface Campaign {
  id: string;
  name: string;
  description: string;
  setting: string;
  ruleset_id: string | null;
  created_at: string;
  updated_at: string;
}

export interface Player {
  id: string;
  campaign_id: string;
  name: string;
  personality: string;
  color: string;
  created_at: string;
}

export interface PlayerCharacter {
  id: string;
  campaign_id: string;
  player_id: string | null;
  name: string;
  pronouns: string;
  details: Record<string, unknown>;
  created_at: string;
}

export interface Message {
  id: string;
  campaign_id: string;
  sender_type: 'gm' | 'player' | 'system';
  sender_id: string | null;
  sender_name: string;
  content: string;
  metadata: MessageMetadata;
  timestamp: string;
}

export interface MessageMetadata {
  tool_calls?: ToolCall[];
  rule_updates?: string[];
}

export interface ToolCall {
  tool: string;
  input: Record<string, unknown>;
  result: string;
}

export interface CampaignLogEntry {
  id: string;
  campaign_id: string;
  entry_type: string;
  summary: string;
  details: Record<string, unknown>;
  timestamp: string;
}

export interface PlayerMemory {
  id: string;
  player_id: string;
  campaign_id: string;
  content: string;
  memory_type: string;
  importance: number;
  created_at: string;
}

export interface GroupMemory {
  id: string;
  campaign_id: string;
  content: string;
  memory_type: string;
  importance: number;
  created_at: string;
}

export interface AppConfig {
  api: {
    groq_api_key: string;
  };
  models: {
    router: string;
    player: string;
    parameters: {
      router_max_tokens: number;
      player_max_tokens: number;
      temperature: number;
    };
  };
  app: {
    theme: string;
    context_messages: number;
  };
}

export interface CreateCampaignRequest {
  name: string;
  description: string;
  setting: string;
  ruleset_id: string | null;
}

export interface CreatePlayerRequest {
  campaign_id: string;
  name: string;
  personality: string;
  color: string;
}

export interface CreatePlayerCharacterRequest {
  campaign_id: string;
  player_id: string | null;
  name: string;
  pronouns: string;
  details: Record<string, unknown>;
}

export interface UpdatePlayerCharacterRequest {
  id: string;
  name: string;
  pronouns: string;
  player_id: string | null;
  details: Record<string, unknown>;
}

export interface Document {
  id: string;
  campaign_id: string;
  name: string;
  content: string;
  created_at: string;
  updated_at: string;
}

export interface CreateDocumentRequest {
  campaign_id: string;
  name: string;
  content: string;
}

export interface UpdateDocumentRequest {
  id: string;
  name: string;
  content: string;
}

export interface MessagePromptData {
  prompt_data: string;
  llm_response: string;
}

export type AppView = 'home' | 'chat' | 'campaign-setup' | 'ruleset-edit';

// Event payloads from Tauri backend
export interface PlayerTypingEvent {
  campaign_id: string;
  player_id: string;
  player_name: string;
  player_color: string;
}

export interface MessageEvent {
  message: Message;
}

export interface GenerationCompleteEvent {
  campaign_id: string;
}

export interface GenerationErrorEvent {
  campaign_id: string;
  error: string;
}
