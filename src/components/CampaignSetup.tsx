import { useState, type FormEvent } from 'react';
import { useCampaign } from '../hooks/useCampaign';

export function CampaignSetup() {
  const { createCampaign } = useCampaign();
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [setting, setSetting] = useState('');
  const [ruleset, setRuleset] = useState('');
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;
    setSubmitting(true);
    try {
      await createCampaign({ name: name.trim(), description, setting, ruleset });
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className="flex-1 flex items-center justify-center p-8">
      <form onSubmit={handleSubmit} className="w-full max-w-lg space-y-4">
        <h2 className="text-xl font-bold mb-6">New Campaign</h2>

        <div>
          <label className="block text-sm text-gray-400 mb-1">Campaign Name *</label>
          <input
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
            placeholder="The Lost Mines of Phandelver"
          />
        </div>

        <div>
          <label className="block text-sm text-gray-400 mb-1">Setting</label>
          <textarea
            value={setting}
            onChange={(e) => setSetting(e.target.value)}
            className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500 h-24 resize-none"
            placeholder="A high fantasy world with medieval technology..."
          />
        </div>

        <div>
          <label className="block text-sm text-gray-400 mb-1">Description</label>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500 h-20 resize-none"
            placeholder="Brief description of the campaign..."
          />
        </div>

        <div>
          <label className="block text-sm text-gray-400 mb-1">Ruleset</label>
          <input
            value={ruleset}
            onChange={(e) => setRuleset(e.target.value)}
            className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
            placeholder="D&D 5e, Pathfinder 2e, etc."
          />
        </div>

        <button
          type="submit"
          disabled={!name.trim() || submitting}
          className="w-full py-2 bg-indigo-600 hover:bg-indigo-500 disabled:bg-gray-700 disabled:text-gray-500 rounded text-sm font-medium transition-colors"
        >
          {submitting ? 'Creating...' : 'Create Campaign'}
        </button>
      </form>
    </div>
  );
}
