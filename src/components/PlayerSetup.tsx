import { useState, useEffect, type FormEvent } from 'react';
import { useCampaign } from '../hooks/useCampaign';
import { useAppState, useAppDispatch } from '../stores/appStore';

const PLAYER_COLORS = ['#6366f1', '#ec4899', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6', '#06b6d4', '#f97316'];

export function PlayerSetupModal() {
  const { createPlayer } = useCampaign();
  const { activeCampaign, playerSetupOpen } = useAppState();
  const dispatch = useAppDispatch();
  const [name, setName] = useState('');
  const [personality, setPersonality] = useState('');
  const [color, setColor] = useState(PLAYER_COLORS[0]);
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    if (playerSetupOpen) {
      setName('');
      setPersonality('');
      setColor(PLAYER_COLORS[0]);
    }
  }, [playerSetupOpen]);

  if (!playerSetupOpen || !activeCampaign) return null;

  const close = () => dispatch({ type: 'SET_PLAYER_SETUP_OPEN', open: false });

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;
    setSubmitting(true);
    try {
      await createPlayer({
        campaign_id: activeCampaign.id,
        name: name.trim(),
        personality,
        color,
      });
      close();
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div className="absolute inset-0 bg-black/60" onClick={close} />
      <div className="relative bg-gray-900 border border-gray-700 rounded-lg shadow-xl w-full max-w-lg mx-4 max-h-[90vh] overflow-y-auto">
        <div className="flex items-center justify-between p-4 border-b border-gray-700">
          <h2 className="text-lg font-bold">Add Player</h2>
          <button onClick={close} className="text-gray-400 hover:text-gray-200 text-xl leading-none">&times;</button>
        </div>

        <form onSubmit={handleSubmit} className="p-4 space-y-4">
          <div>
            <label className="block text-sm text-gray-400 mb-1">Player Name *</label>
            <input
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
              placeholder="Alex"
              autoFocus
            />
          </div>

          <div>
            <label className="block text-sm text-gray-400 mb-1">Personality</label>
            <textarea
              value={personality}
              onChange={(e) => setPersonality(e.target.value)}
              className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500 h-20 resize-none"
              placeholder="How this player tends to approach the game..."
            />
          </div>

          <div>
            <label className="block text-sm text-gray-400 mb-1">Color</label>
            <div className="flex gap-2">
              {PLAYER_COLORS.map((c) => (
                <button
                  key={c}
                  type="button"
                  onClick={() => setColor(c)}
                  className={`w-8 h-8 rounded-full border-2 transition-all ${
                    color === c ? 'border-white scale-110' : 'border-transparent'
                  }`}
                  style={{ backgroundColor: c }}
                />
              ))}
            </div>
          </div>

          <button
            type="submit"
            disabled={!name.trim() || submitting}
            className="w-full py-2 bg-indigo-600 hover:bg-indigo-500 disabled:bg-gray-700 disabled:text-gray-500 rounded text-sm font-medium transition-colors"
          >
            {submitting ? 'Adding...' : 'Add Player'}
          </button>
        </form>
      </div>
    </div>
  );
}
