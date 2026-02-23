import { useCallback, useEffect, useRef } from 'react';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { useAppState, useAppDispatch } from '../stores/appStore';
import * as api from '../lib/api';
import type { MessageEvent, PlayerTypingEvent, GenerationCompleteEvent, GenerationErrorEvent } from '../lib/types';

export function useChat() {
  const state = useAppState();
  const dispatch = useAppDispatch();
  const listenersRef = useRef<UnlistenFn[]>([]);
  const cancelledRef = useRef(false);

  useEffect(() => {
    cancelledRef.current = false;

    const setup = async () => {
      const fns = await Promise.all([
        listen<MessageEvent>('gm:message_stored', (event) => {
          dispatch({ type: 'ADD_MESSAGE', message: event.payload.message });
        }),
        listen<PlayerTypingEvent>('gm:player_typing', (event) => {
          const p = event.payload;
          dispatch({ type: 'SET_TYPING_PLAYER', player: { id: p.player_id, name: p.player_name, color: p.player_color } });
        }),
        listen<MessageEvent>('gm:player_response', (event) => {
          dispatch({ type: 'ADD_MESSAGE', message: event.payload.message });
        }),
        listen<GenerationCompleteEvent>('gm:generation_complete', () => {
          dispatch({ type: 'SET_TYPING_PLAYER', player: null });
          dispatch({ type: 'SET_LOADING', loading: false });
        }),
        listen<GenerationErrorEvent>('gm:generation_error', (event) => {
          dispatch({ type: 'SET_ERROR', error: event.payload.error });
          dispatch({ type: 'SET_TYPING_PLAYER', player: null });
        }),
      ]);

      if (cancelledRef.current) {
        // Cleanup already ran before promises resolved (StrictMode double-mount)
        fns.forEach((fn) => fn());
        return;
      }

      listenersRef.current = fns;
    };

    setup();

    return () => {
      cancelledRef.current = true;
      listenersRef.current.forEach((unlisten) => unlisten());
      listenersRef.current = [];
    };
  }, [dispatch]);

  const sendMessage = useCallback(
    async (content: string) => {
      if (!state.activeCampaign) return;
      dispatch({ type: 'SET_LOADING', loading: true });
      dispatch({ type: 'SET_ERROR', error: null });
      try {
        await api.sendGmMessage(state.activeCampaign.id, content);
      } catch (e) {
        dispatch({ type: 'SET_ERROR', error: String(e) });
        dispatch({ type: 'SET_LOADING', loading: false });
      }
    },
    [state.activeCampaign, dispatch],
  );

  return {
    messages: state.messages,
    loading: state.loading,
    error: state.error,
    typingPlayer: state.typingPlayer,
    sendMessage,
  };
}
