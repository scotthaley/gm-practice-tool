import { useState, type FormEvent } from 'react';
import { useCampaign } from '../hooks/useCampaign';
import { useAppState } from '../stores/appStore';

const PLAYER_COLORS = ['#6366f1', '#ec4899', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6', '#06b6d4', '#f97316'];

export function PlayerSetup() {
  const { createPlayer } = useCampaign();
  const { activeCampaign } = useAppState();
  const [name, setName] = useState('');
  const [race, setRace] = useState('');
  const [charClass, setCharClass] = useState('');
  const [level, setLevel] = useState(1);
  const [backstory, setBackstory] = useState('');
  const [personality, setPersonality] = useState('');
  const [color, setColor] = useState(PLAYER_COLORS[0]);
  const [submitting, setSubmitting] = useState(false);

  if (!activeCampaign) {
    return (
      <div className="flex-1 flex items-center justify-center text-gray-500">
        Select a campaign first
      </div>
    );
  }

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;
    setSubmitting(true);
    try {
      await createPlayer({
        campaign_id: activeCampaign.id,
        name: name.trim(),
        race,
        class: charClass,
        level,
        backstory,
        personality,
        stats: {},
        color,
      });
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className="flex-1 flex items-center justify-center p-8">
      <form onSubmit={handleSubmit} className="w-full max-w-lg space-y-4">
        <h2 className="text-xl font-bold mb-6">Add Player Character</h2>

        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm text-gray-400 mb-1">Character Name *</label>
            <input
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
              placeholder="Thorin Ironforge"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-400 mb-1">Level</label>
            <input
              type="number"
              min={1}
              max={20}
              value={level}
              onChange={(e) => setLevel(parseInt(e.target.value) || 1)}
              className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
            />
          </div>
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm text-gray-400 mb-1">Race</label>
            <input
              value={race}
              onChange={(e) => setRace(e.target.value)}
              className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
              placeholder="Dwarf"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-400 mb-1">Class</label>
            <input
              value={charClass}
              onChange={(e) => setCharClass(e.target.value)}
              className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
              placeholder="Fighter"
            />
          </div>
        </div>

        <div>
          <label className="block text-sm text-gray-400 mb-1">Personality</label>
          <textarea
            value={personality}
            onChange={(e) => setPersonality(e.target.value)}
            className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500 h-20 resize-none"
            placeholder="Gruff but loyal, speaks in short sentences..."
          />
        </div>

        <div>
          <label className="block text-sm text-gray-400 mb-1">Backstory</label>
          <textarea
            value={backstory}
            onChange={(e) => setBackstory(e.target.value)}
            className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500 h-24 resize-none"
            placeholder="Born in the mountain holds..."
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
  );
}
