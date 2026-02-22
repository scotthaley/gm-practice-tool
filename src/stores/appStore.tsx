import { createContext, useContext, useReducer, type Dispatch, type ReactNode } from 'react';
import type { Campaign, Player, Message, AppView } from '../lib/types';

interface AppState {
  campaigns: Campaign[];
  activeCampaign: Campaign | null;
  players: Player[];
  messages: Message[];
  currentView: AppView;
  loading: boolean;
  error: string | null;
}

type AppAction =
  | { type: 'SET_CAMPAIGNS'; campaigns: Campaign[] }
  | { type: 'SET_ACTIVE_CAMPAIGN'; campaign: Campaign | null }
  | { type: 'ADD_CAMPAIGN'; campaign: Campaign }
  | { type: 'SET_PLAYERS'; players: Player[] }
  | { type: 'ADD_PLAYER'; player: Player }
  | { type: 'SET_MESSAGES'; messages: Message[] }
  | { type: 'ADD_MESSAGES'; messages: Message[] }
  | { type: 'SET_VIEW'; view: AppView }
  | { type: 'SET_LOADING'; loading: boolean }
  | { type: 'SET_ERROR'; error: string | null };

const initialState: AppState = {
  campaigns: [],
  activeCampaign: null,
  players: [],
  messages: [],
  currentView: 'chat',
  loading: false,
  error: null,
};

function appReducer(state: AppState, action: AppAction): AppState {
  switch (action.type) {
    case 'SET_CAMPAIGNS':
      return { ...state, campaigns: action.campaigns };
    case 'SET_ACTIVE_CAMPAIGN':
      return { ...state, activeCampaign: action.campaign, messages: [], players: [] };
    case 'ADD_CAMPAIGN':
      return { ...state, campaigns: [...state.campaigns, action.campaign] };
    case 'SET_PLAYERS':
      return { ...state, players: action.players };
    case 'ADD_PLAYER':
      return { ...state, players: [...state.players, action.player] };
    case 'SET_MESSAGES':
      return { ...state, messages: action.messages };
    case 'ADD_MESSAGES':
      return { ...state, messages: [...state.messages, ...action.messages] };
    case 'SET_VIEW':
      return { ...state, currentView: action.view };
    case 'SET_LOADING':
      return { ...state, loading: action.loading };
    case 'SET_ERROR':
      return { ...state, error: action.error };
    default:
      return state;
  }
}

const AppStateContext = createContext<AppState>(initialState);
const AppDispatchContext = createContext<Dispatch<AppAction>>(() => {});

export function AppProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(appReducer, initialState);
  return (
    <AppStateContext.Provider value={state}>
      <AppDispatchContext.Provider value={dispatch}>
        {children}
      </AppDispatchContext.Provider>
    </AppStateContext.Provider>
  );
}

export function useAppState() {
  return useContext(AppStateContext);
}

export function useAppDispatch() {
  return useContext(AppDispatchContext);
}
