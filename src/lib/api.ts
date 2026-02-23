import { invoke } from '@tauri-apps/api/core';
import type {
  AppConfig,
  Campaign,
  CampaignLogEntry,
  CreateCampaignRequest,
  CreatePlayerRequest,
  CreatePlayerCharacterRequest,
  Message,
  Player,
  PlayerCharacter,
  Ruleset,
} from './types';

export async function getConfig(): Promise<AppConfig> {
  return invoke('get_config');
}

export async function updateConfig(config: AppConfig): Promise<void> {
  return invoke('update_config', { config });
}

export async function createCampaign(request: CreateCampaignRequest): Promise<Campaign> {
  return invoke('create_campaign', { request });
}

export async function listCampaigns(): Promise<Campaign[]> {
  return invoke('list_campaigns');
}

export async function getCampaign(campaignId: string): Promise<Campaign> {
  return invoke('get_campaign', { campaignId });
}

export async function createPlayer(request: CreatePlayerRequest): Promise<Player> {
  return invoke('create_player', { request });
}

export async function listPlayers(campaignId: string): Promise<Player[]> {
  return invoke('list_players', { campaignId });
}

export async function createPlayerCharacter(request: CreatePlayerCharacterRequest): Promise<PlayerCharacter> {
  return invoke('create_player_character', { request });
}

export async function listPlayerCharacters(campaignId: string): Promise<PlayerCharacter[]> {
  return invoke('list_player_characters', { campaignId });
}

export async function getMessages(campaignId: string): Promise<Message[]> {
  return invoke('get_messages', { campaignId });
}

export async function sendGmMessage(campaignId: string, message: string): Promise<Message[]> {
  return invoke('send_gm_message', { campaignId, message });
}

export async function getCampaignLog(campaignId: string): Promise<CampaignLogEntry[]> {
  return invoke('get_campaign_log', { campaignId });
}

export async function createRuleset(request: { name: string; content: string }): Promise<Ruleset> {
  return invoke('create_ruleset', { request });
}

export async function listRulesets(): Promise<Ruleset[]> {
  return invoke('list_rulesets');
}

export async function getRuleset(rulesetId: string): Promise<Ruleset> {
  return invoke('get_ruleset', { rulesetId });
}

export async function updateRuleset(request: { id: string; name: string; content: string }): Promise<Ruleset> {
  return invoke('update_ruleset', { request });
}

export async function deleteRuleset(rulesetId: string): Promise<void> {
  return invoke('delete_ruleset', { rulesetId });
}
