import type { Player } from '../lib/types';

export function PlayerCard({ player, onClick }: { player: Player; onClick: () => void }) {
  return (
    <button
      onClick={onClick}
      className="w-full text-left rounded border border-gray-700 bg-gray-800/50 overflow-hidden px-3 py-2 hover:bg-gray-700/50 transition-colors"
      style={{ borderLeftColor: player.color, borderLeftWidth: '3px' }}
    >
      <span className="text-sm font-medium" style={{ color: player.color }}>
        {player.name}
      </span>
    </button>
  );
}
