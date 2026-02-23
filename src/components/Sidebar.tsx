import { useState } from 'react';
import { useAppState, useAppDispatch } from '../stores/appStore';
import { PlayerCard } from './PlayerCard';
import { PlayerDetailModal } from './PlayerDetailModal';
import { GroupMemoryModal } from './GroupMemoryModal';
import type { Player, PlayerCharacter } from '../lib/types';

function CharacterCard({ character, playerName }: { character: PlayerCharacter; playerName: string | null }) {
  const race = character.details.race as string | undefined;
  const charClass = character.details.class as string | undefined;
  const level = character.details.level as number | undefined;
  const summary = [race, charClass, level ? `L${level}` : null].filter(Boolean).join(' ');

  return (
    <div className="rounded border border-gray-700 bg-gray-800/50 px-3 py-2">
      <div className="flex items-center justify-between">
        <span className="text-sm font-medium text-gray-200">{character.name}</span>
        {playerName ? (
          <span className="text-xs text-gray-500">{playerName}</span>
        ) : (
          <span className="text-xs text-amber-500/80">NPC</span>
        )}
      </div>
      {summary && <p className="text-xs text-gray-500 mt-0.5">{summary}</p>}
    </div>
  );
}

export function Sidebar() {
  const { activeCampaign, players, playerCharacters, documents, currentView } = useAppState();
  const dispatch = useAppDispatch();
  const [selectedPlayer, setSelectedPlayer] = useState<Player | null>(null);
  const [groupMemoryOpen, setGroupMemoryOpen] = useState(false);

  const inCampaign = currentView === 'chat';

  const getPlayerName = (playerId: string | null) => {
    if (!playerId) return null;
    return players.find((p) => p.id === playerId)?.name ?? null;
  };

  return (
    <aside className="w-72 bg-gray-800 border-r border-gray-700 flex flex-col h-full">
      <div className="p-4 border-b border-gray-700">
        {inCampaign ? (
          <button
            onClick={() => {
              dispatch({ type: 'SET_ACTIVE_CAMPAIGN', campaign: null });
              dispatch({ type: 'SET_VIEW', view: 'home' });
            }}
            className="text-lg font-bold text-indigo-400 hover:text-indigo-300 transition-colors"
          >
            GM Practice Tool
          </button>
        ) : (
          <h1 className="text-lg font-bold text-indigo-400">GM Practice Tool</h1>
        )}
      </div>

      {activeCampaign && (
        <div className="p-3 flex-1 overflow-y-auto space-y-4">
          {/* Players */}
          <div>
            <div className="flex items-center justify-between mb-2">
              <h2 className="text-xs font-semibold text-gray-400 uppercase tracking-wider">Players</h2>
              <button
                onClick={() => dispatch({ type: 'SET_PLAYER_SETUP_OPEN', open: true })}
                className="text-xs text-indigo-400 hover:text-indigo-300"
              >
                + Add
              </button>
            </div>
            <div className="space-y-2">
              {players.map((p) => (
                <PlayerCard key={p.id} player={p} onClick={() => setSelectedPlayer(p)} />
              ))}
              {players.length === 0 && (
                <p className="text-xs text-gray-500 px-3 py-2">No players yet</p>
              )}
            </div>
          </div>

          {/* Group Memory */}
          <button
            onClick={() => setGroupMemoryOpen(true)}
            className="w-full text-left px-3 py-2 rounded text-sm transition-colors text-gray-400 hover:bg-gray-700 hover:text-gray-200 border border-gray-700"
          >
            View Group Memory
          </button>

          {/* Documents */}
          <div>
            <div className="flex items-center justify-between mb-2">
              <h2 className="text-xs font-semibold text-gray-400 uppercase tracking-wider">Documents</h2>
              <button
                onClick={() => {
                  dispatch({ type: 'SET_EDITING_DOCUMENT', document: null });
                  dispatch({ type: 'SET_DOCUMENT_MODAL_OPEN', open: true });
                }}
                className="text-xs text-indigo-400 hover:text-indigo-300"
              >
                + Add
              </button>
            </div>
            <div className="space-y-2">
              {documents.map((doc) => (
                <button
                  key={doc.id}
                  onClick={() => {
                    dispatch({ type: 'SET_EDITING_DOCUMENT', document: doc });
                    dispatch({ type: 'SET_DOCUMENT_MODAL_OPEN', open: true });
                  }}
                  className="w-full text-left rounded border border-gray-700 bg-gray-800/50 px-3 py-2 hover:border-gray-600 transition-colors"
                >
                  <span className="text-sm font-medium text-gray-200">{doc.name}</span>
                  <p className="text-xs text-gray-500 mt-0.5 truncate">{doc.content || 'Empty'}</p>
                </button>
              ))}
              {documents.length === 0 && (
                <p className="text-xs text-gray-500 px-3 py-2">No documents yet</p>
              )}
            </div>
          </div>

          {/* Characters */}
          <div>
            <div className="flex items-center justify-between mb-2">
              <h2 className="text-xs font-semibold text-gray-400 uppercase tracking-wider">Characters</h2>
              <button
                onClick={() => dispatch({ type: 'SET_CHARACTER_SETUP_OPEN', open: true })}
                className="text-xs text-indigo-400 hover:text-indigo-300"
              >
                + Add
              </button>
            </div>
            <div className="space-y-2">
              {playerCharacters.map((c) => (
                <CharacterCard
                  key={c.id}
                  character={c}
                  playerName={getPlayerName(c.player_id)}
                />
              ))}
              {playerCharacters.length === 0 && (
                <p className="text-xs text-gray-500 px-3 py-2">No characters yet</p>
              )}
            </div>
          </div>
        </div>
      )}

      {/* Nav */}
      {!inCampaign && (
        <div className="p-3 border-t border-gray-700 space-y-1">
          <button
            onClick={() => dispatch({ type: 'SET_VIEW', view: 'home' })}
            className={`w-full text-left px-3 py-2 rounded text-sm transition-colors ${
              currentView === 'home'
                ? 'bg-gray-700 text-gray-100'
                : 'text-gray-400 hover:bg-gray-700 hover:text-gray-200'
            }`}
          >
            Campaigns
          </button>
          <button
            onClick={() => dispatch({ type: 'SET_VIEW', view: 'ruleset-edit' })}
            className={`w-full text-left px-3 py-2 rounded text-sm transition-colors ${
              currentView === 'ruleset-edit'
                ? 'bg-gray-700 text-gray-100'
                : 'text-gray-400 hover:bg-gray-700 hover:text-gray-200'
            }`}
          >
            Rulesets
          </button>
        </div>
      )}

      <div className="flex-1" />

      {/* Settings - always at bottom */}
      <div className="p-3 border-t border-gray-700">
        <button
          onClick={() => dispatch({ type: 'SET_SETTINGS_OPEN', open: true })}
          className="w-full text-left px-3 py-2 rounded text-sm transition-colors text-gray-400 hover:bg-gray-700 hover:text-gray-200"
        >
          Settings
        </button>
      </div>
      {selectedPlayer && activeCampaign && (
        <PlayerDetailModal
          player={selectedPlayer}
          campaignId={activeCampaign.id}
          onClose={() => setSelectedPlayer(null)}
        />
      )}
      {groupMemoryOpen && activeCampaign && (
        <GroupMemoryModal
          campaignId={activeCampaign.id}
          onClose={() => setGroupMemoryOpen(false)}
        />
      )}
    </aside>
  );
}
