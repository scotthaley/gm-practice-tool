import { useState } from 'react';
import type { Player } from '../lib/types';

export function PlayerCard({ player }: { player: Player }) {
  const [expanded, setExpanded] = useState(false);

  return (
    <div
      className="rounded border border-gray-700 bg-gray-800/50 overflow-hidden"
      style={{ borderLeftColor: player.color, borderLeftWidth: '3px' }}
    >
      <button
        onClick={() => setExpanded(!expanded)}
        className="w-full text-left px-3 py-2 flex items-center justify-between"
      >
        <div>
          <span className="text-sm font-medium" style={{ color: player.color }}>
            {player.name}
          </span>
          <span className="text-xs text-gray-400 ml-2">
            L{player.level} {player.race} {player.class}
          </span>
        </div>
        <span className="text-gray-500 text-xs">{expanded ? '\u25B2' : '\u25BC'}</span>
      </button>
      {expanded && (
        <div className="px-3 pb-2 text-xs text-gray-400 space-y-1">
          {player.personality && <p><strong>Personality:</strong> {player.personality}</p>}
          {player.backstory && <p><strong>Backstory:</strong> {player.backstory}</p>}
        </div>
      )}
    </div>
  );
}
