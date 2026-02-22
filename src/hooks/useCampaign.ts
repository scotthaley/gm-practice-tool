import { useCallback, useEffect } from 'react';
import { useAppState, useAppDispatch } from '../stores/appStore';
import * as api from '../lib/api';
import type { CreateCampaignRequest, CreatePlayerRequest } from '../lib/types';

export function useCampaign() {
  const state = useAppState();
  const dispatch = useAppDispatch();

  const loadCampaigns = useCallback(async () => {
    try {
      const campaigns = await api.listCampaigns();
      dispatch({ type: 'SET_CAMPAIGNS', campaigns });
    } catch (e) {
      dispatch({ type: 'SET_ERROR', error: String(e) });
    }
  }, [dispatch]);

  const selectCampaign = useCallback(
    async (campaignId: string) => {
      try {
        const campaign = await api.getCampaign(campaignId);
        dispatch({ type: 'SET_ACTIVE_CAMPAIGN', campaign });
        const players = await api.listPlayers(campaignId);
        dispatch({ type: 'SET_PLAYERS', players });
        const messages = await api.getMessages(campaignId);
        dispatch({ type: 'SET_MESSAGES', messages });
        dispatch({ type: 'SET_VIEW', view: 'chat' });
      } catch (e) {
        dispatch({ type: 'SET_ERROR', error: String(e) });
      }
    },
    [dispatch],
  );

  const createCampaign = useCallback(
    async (request: CreateCampaignRequest) => {
      try {
        const campaign = await api.createCampaign(request);
        dispatch({ type: 'ADD_CAMPAIGN', campaign });
        dispatch({ type: 'SET_ACTIVE_CAMPAIGN', campaign });
        dispatch({ type: 'SET_VIEW', view: 'chat' });
        return campaign;
      } catch (e) {
        dispatch({ type: 'SET_ERROR', error: String(e) });
        throw e;
      }
    },
    [dispatch],
  );

  const createPlayer = useCallback(
    async (request: CreatePlayerRequest) => {
      try {
        const player = await api.createPlayer(request);
        dispatch({ type: 'ADD_PLAYER', player });
        dispatch({ type: 'SET_VIEW', view: 'chat' });
        return player;
      } catch (e) {
        dispatch({ type: 'SET_ERROR', error: String(e) });
        throw e;
      }
    },
    [dispatch],
  );

  useEffect(() => {
    loadCampaigns();
  }, [loadCampaigns]);

  return {
    campaigns: state.campaigns,
    activeCampaign: state.activeCampaign,
    players: state.players,
    loadCampaigns,
    selectCampaign,
    createCampaign,
    createPlayer,
  };
}
