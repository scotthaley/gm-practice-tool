import { useState, useEffect, type FormEvent } from 'react';
import { useCampaign } from '../hooks/useCampaign';
import { useAppState, useAppDispatch } from '../stores/appStore';

export function CharacterSetupModal() {
  const { createPlayerCharacter, players } = useCampaign();
  const { activeCampaign, characterSetupOpen } = useAppState();
  const dispatch = useAppDispatch();
  const [name, setName] = useState('');
  const [pronouns, setPronouns] = useState('');
  const [playerId, setPlayerId] = useState<string>('');
  const [detailsText, setDetailsText] = useState('{}');
  const [detailsError, setDetailsError] = useState('');
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    if (characterSetupOpen) {
      setName('');
      setPronouns('');
      setPlayerId('');
      setDetailsText('{}');
      setDetailsError('');
    }
  }, [characterSetupOpen]);

  if (!characterSetupOpen || !activeCampaign) return null;

  const close = () => dispatch({ type: 'SET_CHARACTER_SETUP_OPEN', open: false });

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;
    let details: Record<string, unknown>;
    try {
      details = JSON.parse(detailsText);
      if (typeof details !== 'object' || Array.isArray(details) || details === null) {
        setDetailsError('Must be a JSON object');
        return;
      }
    } catch {
      setDetailsError('Invalid JSON');
      return;
    }
    setSubmitting(true);
    try {
      await createPlayerCharacter({
        campaign_id: activeCampaign.id,
        player_id: playerId || null,
        name: name.trim(),
        pronouns: pronouns.trim(),
        details,
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
          <h2 className="text-lg font-bold">Add Character</h2>
          <button onClick={close} className="text-gray-400 hover:text-gray-200 text-xl leading-none">&times;</button>
        </div>

        <form onSubmit={handleSubmit} className="p-4 space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm text-gray-400 mb-1">Character Name *</label>
              <input
                value={name}
                onChange={(e) => setName(e.target.value)}
                className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
                placeholder="Thorin Ironforge"
                autoFocus
              />
            </div>
            <div>
              <label className="block text-sm text-gray-400 mb-1">Pronouns</label>
              <input
                value={pronouns}
                onChange={(e) => setPronouns(e.target.value)}
                className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
                placeholder="she/her"
              />
            </div>
          </div>

          <div>
            <label className="block text-sm text-gray-400 mb-1">Controlled By</label>
            <select
              value={playerId}
              onChange={(e) => setPlayerId(e.target.value)}
              className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
            >
              <option value="">NPC (no player)</option>
              {players.map((p) => (
                <option key={p.id} value={p.id}>{p.name}</option>
              ))}
            </select>
          </div>

          <div>
            <label className="block text-sm text-gray-400 mb-1">Details (JSON)</label>
            <textarea
              value={detailsText}
              onChange={(e) => { setDetailsText(e.target.value); setDetailsError(''); }}
              className={`w-full bg-gray-800 border rounded px-3 py-2 text-sm font-mono focus:outline-none h-32 resize-y ${
                detailsError ? 'border-red-500' : 'border-gray-600 focus:border-indigo-500'
              }`}
              placeholder='{"race": "Dwarf", "class": "Fighter", "level": 5, "backstory": "..."}'
            />
            {detailsError && <p className="text-red-400 text-xs mt-1">{detailsError}</p>}
          </div>

          <button
            type="submit"
            disabled={!name.trim() || submitting}
            className="w-full py-2 bg-indigo-600 hover:bg-indigo-500 disabled:bg-gray-700 disabled:text-gray-500 rounded text-sm font-medium transition-colors"
          >
            {submitting ? 'Adding...' : 'Add Character'}
          </button>
        </form>
      </div>
    </div>
  );
}
