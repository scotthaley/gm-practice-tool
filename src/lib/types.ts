export interface Campaign {
  id: string;
  name: string;
  description: string;
  setting: string;
  ruleset: string;
  created_at: string;
  updated_at: string;
}

export interface Player {
  id: string;
  campaign_id: string;
  name: string;
  race: string;
  class: string;
  level: number;
  backstory: string;
  personality: string;
  stats: Record<string, number>;
  color: string;
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

export interface AppConfig {
  api: {
    anthropic_api_key: string;
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
  };
}

export interface CreateCampaignRequest {
  name: string;
  description: string;
  setting: string;
  ruleset: string;
}

export interface CreatePlayerRequest {
  campaign_id: string;
  name: string;
  race: string;
  class: string;
  level: number;
  backstory: string;
  personality: string;
  stats: Record<string, number>;
  color: string;
}

export type AppView = 'chat' | 'campaign-setup' | 'player-setup' | 'settings';
