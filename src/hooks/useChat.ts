import { useCallback } from 'react';
import { useAppState, useAppDispatch } from '../stores/appStore';
import * as api from '../lib/api';

export function useChat() {
  const state = useAppState();
  const dispatch = useAppDispatch();

  const sendMessage = useCallback(
    async (content: string) => {
      if (!state.activeCampaign) return;
      dispatch({ type: 'SET_LOADING', loading: true });
      dispatch({ type: 'SET_ERROR', error: null });
      try {
        const newMessages = await api.sendGmMessage(state.activeCampaign.id, content);
        dispatch({ type: 'ADD_MESSAGES', messages: newMessages });
      } catch (e) {
        dispatch({ type: 'SET_ERROR', error: String(e) });
      } finally {
        dispatch({ type: 'SET_LOADING', loading: false });
      }
    },
    [state.activeCampaign, dispatch],
  );

  return {
    messages: state.messages,
    loading: state.loading,
    error: state.error,
    sendMessage,
  };
}
