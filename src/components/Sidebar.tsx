import { useCampaign } from '../hooks/useCampaign';
import { useAppState, useAppDispatch } from '../stores/appStore';
import { PlayerCard } from './PlayerCard';

export function Sidebar() {
  const { campaigns, activeCampaign, players, selectCampaign } = useCampaign();
  const { currentView } = useAppState();
  const dispatch = useAppDispatch();

  return (
    <aside className="w-72 bg-gray-800 border-r border-gray-700 flex flex-col h-full">
      <div className="p-4 border-b border-gray-700">
        <h1 className="text-lg font-bold text-indigo-400">GM Practice Tool</h1>
      </div>

      {/* Campaigns */}
      <div className="p-3">
        <div className="flex items-center justify-between mb-2">
          <h2 className="text-xs font-semibold text-gray-400 uppercase tracking-wider">Campaigns</h2>
          <button
            onClick={() => dispatch({ type: 'SET_VIEW', view: 'campaign-setup' })}
            className="text-xs text-indigo-400 hover:text-indigo-300"
          >
            + New
          </button>
        </div>
        <div className="space-y-1">
          {campaigns.map((c) => (
            <button
              key={c.id}
              onClick={() => selectCampaign(c.id)}
              className={`w-full text-left px-3 py-2 rounded text-sm transition-colors ${
                activeCampaign?.id === c.id
                  ? 'bg-indigo-600/20 text-indigo-300'
                  : 'text-gray-300 hover:bg-gray-700'
              }`}
            >
              {c.name}
            </button>
          ))}
          {campaigns.length === 0 && (
            <p className="text-xs text-gray-500 px-3 py-2">No campaigns yet</p>
          )}
        </div>
      </div>

      {/* Players */}
      {activeCampaign && (
        <div className="p-3 flex-1 overflow-y-auto">
          <div className="flex items-center justify-between mb-2">
            <h2 className="text-xs font-semibold text-gray-400 uppercase tracking-wider">Players</h2>
            <button
              onClick={() => dispatch({ type: 'SET_VIEW', view: 'player-setup' })}
              className="text-xs text-indigo-400 hover:text-indigo-300"
            >
              + Add
            </button>
          </div>
          <div className="space-y-2">
            {players.map((p) => (
              <PlayerCard key={p.id} player={p} />
            ))}
            {players.length === 0 && (
              <p className="text-xs text-gray-500 px-3 py-2">No players yet</p>
            )}
          </div>
        </div>
      )}

      {/* Nav */}
      <div className="p-3 border-t border-gray-700">
        <button
          onClick={() => dispatch({ type: 'SET_VIEW', view: 'settings' })}
          className={`w-full text-left px-3 py-2 rounded text-sm transition-colors ${
            currentView === 'settings'
              ? 'bg-gray-700 text-gray-100'
              : 'text-gray-400 hover:bg-gray-700 hover:text-gray-200'
          }`}
        >
          Settings
        </button>
      </div>
    </aside>
  );
}
