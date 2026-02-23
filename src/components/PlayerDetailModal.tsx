import { useState, useEffect } from 'react';
import type { Player, PlayerMemory } from '../lib/types';
import * as api from '../lib/api';

export function PlayerDetailModal({
  player,
  campaignId,
  onClose,
}: {
  player: Player;
  campaignId: string;
  onClose: () => void;
}) {
  const [memories, setMemories] = useState<PlayerMemory[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    api.getPlayerMemories(player.id, campaignId).then((m) => {
      setMemories(m);
      setLoading(false);
    });
  }, [player.id, campaignId]);

  const handleDeleteMemory = async (memoryId: string) => {
    await api.deletePlayerMemory(memoryId);
    setMemories((prev) => prev.filter((m) => m.id !== memoryId));
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div className="absolute inset-0 bg-black/60" onClick={onClose} />
      <div className="relative bg-gray-900 border border-gray-700 rounded-lg shadow-xl w-full max-w-lg mx-4 max-h-[90vh] overflow-y-auto">
        <div className="flex items-center justify-between p-4 border-b border-gray-700">
          <h2 className="text-lg font-bold" style={{ color: player.color }}>
            {player.name}
          </h2>
          <button onClick={onClose} className="text-gray-400 hover:text-gray-200 text-xl leading-none">
            &times;
          </button>
        </div>

        <div className="p-4 space-y-4">
          {player.personality && (
            <div>
              <h3 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-1">Personality</h3>
              <p className="text-sm text-gray-300">{player.personality}</p>
            </div>
          )}

          <div>
            <h3 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-2">Memories</h3>
            {loading ? (
              <p className="text-xs text-gray-500">Loading memories...</p>
            ) : memories.length === 0 ? (
              <p className="text-xs text-gray-500">No memories yet</p>
            ) : (
              <div className="space-y-2">
                {memories.map((m) => (
                  <div key={m.id} className="group rounded border border-gray-700 bg-gray-800/50 px-3 py-2">
                    <div className="flex items-start justify-between gap-2">
                      <p className="text-sm text-gray-300">{m.content}</p>
                      <button
                        onClick={() => handleDeleteMemory(m.id)}
                        className="hidden group-hover:block shrink-0 text-gray-500 hover:text-red-400 text-xs px-1 transition-colors"
                        title="Delete memory"
                      >
                        &times;
                      </button>
                    </div>
                    <div className="flex items-center gap-2 mt-1">
                      <span className="text-xs text-gray-500">{m.memory_type}</span>
                      <span className="text-xs text-gray-600">importance: {m.importance.toFixed(1)}</span>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
