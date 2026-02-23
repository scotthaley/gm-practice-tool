import { createContext, useContext, useReducer, type Dispatch, type ReactNode } from 'react';
import type { Campaign, Player, PlayerCharacter, Message, AppView, Ruleset, Document } from '../lib/types';

interface AppState {
  campaigns: Campaign[];
  activeCampaign: Campaign | null;
  players: Player[];
  playerCharacters: PlayerCharacter[];
  messages: Message[];
  rulesets: Ruleset[];
  documents: Document[];
  documentModalOpen: boolean;
  editingDocument: Document | null;
  currentView: AppView;
  settingsOpen: boolean;
  playerSetupOpen: boolean;
  characterSetupOpen: boolean;
  loading: boolean;
  error: string | null;
}

type AppAction =
  | { type: 'SET_CAMPAIGNS'; campaigns: Campaign[] }
  | { type: 'SET_ACTIVE_CAMPAIGN'; campaign: Campaign | null }
  | { type: 'ADD_CAMPAIGN'; campaign: Campaign }
  | { type: 'SET_PLAYERS'; players: Player[] }
  | { type: 'ADD_PLAYER'; player: Player }
  | { type: 'SET_PLAYER_CHARACTERS'; playerCharacters: PlayerCharacter[] }
  | { type: 'ADD_PLAYER_CHARACTER'; playerCharacter: PlayerCharacter }
  | { type: 'SET_MESSAGES'; messages: Message[] }
  | { type: 'ADD_MESSAGES'; messages: Message[] }
  | { type: 'SET_VIEW'; view: AppView }
  | { type: 'SET_SETTINGS_OPEN'; open: boolean }
  | { type: 'SET_PLAYER_SETUP_OPEN'; open: boolean }
  | { type: 'SET_CHARACTER_SETUP_OPEN'; open: boolean }
  | { type: 'SET_LOADING'; loading: boolean }
  | { type: 'SET_ERROR'; error: string | null }
  | { type: 'SET_RULESETS'; rulesets: Ruleset[] }
  | { type: 'ADD_RULESET'; ruleset: Ruleset }
  | { type: 'UPDATE_RULESET'; ruleset: Ruleset }
  | { type: 'DELETE_RULESET'; rulesetId: string }
  | { type: 'SET_DOCUMENTS'; documents: Document[] }
  | { type: 'ADD_DOCUMENT'; document: Document }
  | { type: 'UPDATE_DOCUMENT'; document: Document }
  | { type: 'DELETE_DOCUMENT'; documentId: string }
  | { type: 'SET_DOCUMENT_MODAL_OPEN'; open: boolean }
  | { type: 'SET_EDITING_DOCUMENT'; document: Document | null }
  | { type: 'DELETE_MESSAGE'; messageId: string };

const initialState: AppState = {
  campaigns: [],
  activeCampaign: null,
  players: [],
  playerCharacters: [],
  messages: [],
  rulesets: [],
  documents: [],
  documentModalOpen: false,
  editingDocument: null,
  currentView: 'home',
  settingsOpen: false,
  playerSetupOpen: false,
  characterSetupOpen: false,
  loading: false,
  error: null,
};

function appReducer(state: AppState, action: AppAction): AppState {
  switch (action.type) {
    case 'SET_CAMPAIGNS':
      return { ...state, campaigns: action.campaigns };
    case 'SET_ACTIVE_CAMPAIGN':
      return { ...state, activeCampaign: action.campaign, messages: [], players: [], playerCharacters: [], documents: [] };
    case 'ADD_CAMPAIGN':
      return { ...state, campaigns: [...state.campaigns, action.campaign] };
    case 'SET_PLAYERS':
      return { ...state, players: action.players };
    case 'ADD_PLAYER':
      return { ...state, players: [...state.players, action.player] };
    case 'SET_PLAYER_CHARACTERS':
      return { ...state, playerCharacters: action.playerCharacters };
    case 'ADD_PLAYER_CHARACTER':
      return { ...state, playerCharacters: [...state.playerCharacters, action.playerCharacter] };
    case 'SET_MESSAGES':
      return { ...state, messages: action.messages };
    case 'ADD_MESSAGES':
      return { ...state, messages: [...state.messages, ...action.messages] };
    case 'SET_VIEW':
      return { ...state, currentView: action.view };
    case 'SET_SETTINGS_OPEN':
      return { ...state, settingsOpen: action.open };
    case 'SET_PLAYER_SETUP_OPEN':
      return { ...state, playerSetupOpen: action.open };
    case 'SET_CHARACTER_SETUP_OPEN':
      return { ...state, characterSetupOpen: action.open };
    case 'SET_LOADING':
      return { ...state, loading: action.loading };
    case 'SET_ERROR':
      return { ...state, error: action.error };
    case 'SET_RULESETS':
      return { ...state, rulesets: action.rulesets };
    case 'ADD_RULESET':
      return { ...state, rulesets: [...state.rulesets, action.ruleset] };
    case 'UPDATE_RULESET':
      return {
        ...state,
        rulesets: state.rulesets.map((r) =>
          r.id === action.ruleset.id ? action.ruleset : r,
        ),
      };
    case 'DELETE_RULESET':
      return {
        ...state,
        rulesets: state.rulesets.filter((r) => r.id !== action.rulesetId),
      };
    case 'SET_DOCUMENTS':
      return { ...state, documents: action.documents };
    case 'ADD_DOCUMENT':
      return { ...state, documents: [...state.documents, action.document] };
    case 'UPDATE_DOCUMENT':
      return {
        ...state,
        documents: state.documents.map((d) =>
          d.id === action.document.id ? action.document : d,
        ),
      };
    case 'DELETE_DOCUMENT':
      return {
        ...state,
        documents: state.documents.filter((d) => d.id !== action.documentId),
      };
    case 'SET_DOCUMENT_MODAL_OPEN':
      return { ...state, documentModalOpen: action.open };
    case 'SET_EDITING_DOCUMENT':
      return { ...state, editingDocument: action.document };
    case 'DELETE_MESSAGE':
      return { ...state, messages: state.messages.filter((m) => m.id !== action.messageId) };
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
