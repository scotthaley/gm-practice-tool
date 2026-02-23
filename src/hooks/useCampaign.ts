import { useCallback, useEffect } from 'react';
import { useAppState, useAppDispatch } from '../stores/appStore';
import * as api from '../lib/api';
import type { CreateCampaignRequest, CreateDocumentRequest, CreatePlayerRequest, CreatePlayerCharacterRequest, UpdateDocumentRequest } from '../lib/types';

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
        const [players, playerCharacters, messages, documents] = await Promise.all([
          api.listPlayers(campaignId),
          api.listPlayerCharacters(campaignId),
          api.getMessages(campaignId),
          api.listDocuments(campaignId),
        ]);
        dispatch({ type: 'SET_PLAYERS', players });
        dispatch({ type: 'SET_PLAYER_CHARACTERS', playerCharacters });
        dispatch({ type: 'SET_MESSAGES', messages });
        dispatch({ type: 'SET_DOCUMENTS', documents });
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

  const createPlayerCharacter = useCallback(
    async (request: CreatePlayerCharacterRequest) => {
      try {
        const playerCharacter = await api.createPlayerCharacter(request);
        dispatch({ type: 'ADD_PLAYER_CHARACTER', playerCharacter });
        return playerCharacter;
      } catch (e) {
        dispatch({ type: 'SET_ERROR', error: String(e) });
        throw e;
      }
    },
    [dispatch],
  );

  const createDocument = useCallback(
    async (request: CreateDocumentRequest) => {
      try {
        const document = await api.createDocument(request);
        dispatch({ type: 'ADD_DOCUMENT', document });
        return document;
      } catch (e) {
        dispatch({ type: 'SET_ERROR', error: String(e) });
        throw e;
      }
    },
    [dispatch],
  );

  const updateDocument = useCallback(
    async (request: UpdateDocumentRequest) => {
      try {
        const document = await api.updateDocument(request);
        dispatch({ type: 'UPDATE_DOCUMENT', document });
        return document;
      } catch (e) {
        dispatch({ type: 'SET_ERROR', error: String(e) });
        throw e;
      }
    },
    [dispatch],
  );

  const deleteDocument = useCallback(
    async (documentId: string) => {
      try {
        await api.deleteDocument(documentId);
        dispatch({ type: 'DELETE_DOCUMENT', documentId });
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
    playerCharacters: state.playerCharacters,
    loadCampaigns,
    selectCampaign,
    createCampaign,
    createPlayer,
    createPlayerCharacter,
    createDocument,
    updateDocument,
    deleteDocument,
  };
}
