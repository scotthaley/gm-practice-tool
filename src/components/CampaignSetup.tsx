import { useState, type FormEvent } from 'react';
import { useCampaign } from '../hooks/useCampaign';
import { useRuleset } from '../hooks/useRuleset';
import { useAppDispatch } from '../stores/appStore';

export function CampaignSetup() {
  const { createCampaign } = useCampaign();
  const { rulesets, createRuleset } = useRuleset();
  const dispatch = useAppDispatch();
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [setting, setSetting] = useState('');
  const [rulesetId, setRulesetId] = useState<string | null>(null);
  const [showNewRuleset, setShowNewRuleset] = useState(false);
  const [newRulesetName, setNewRulesetName] = useState('');
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;
    setSubmitting(true);
    try {
      let finalRulesetId = rulesetId;

      // Create inline ruleset if needed
      if (showNewRuleset && newRulesetName.trim()) {
        const rs = await createRuleset(newRulesetName.trim(), '');
        finalRulesetId = rs.id;
      }

      await createCampaign({
        name: name.trim(),
        description,
        setting,
        ruleset_id: finalRulesetId,
      });
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
          {!showNewRuleset ? (
            <div className="space-y-2">
              <select
                value={rulesetId ?? ''}
                onChange={(e) => setRulesetId(e.target.value || null)}
                className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
              >
                <option value="">None</option>
                {rulesets.map((r) => (
                  <option key={r.id} value={r.id}>
                    {r.name}
                  </option>
                ))}
              </select>
              <div className="flex gap-2">
                <button
                  type="button"
                  onClick={() => setShowNewRuleset(true)}
                  className="text-xs text-indigo-400 hover:text-indigo-300"
                >
                  + Create new ruleset
                </button>
                {rulesets.length > 0 && (
                  <button
                    type="button"
                    onClick={() => dispatch({ type: 'SET_VIEW', view: 'ruleset-edit' })}
                    className="text-xs text-gray-400 hover:text-gray-300"
                  >
                    Edit rulesets
                  </button>
                )}
              </div>
            </div>
          ) : (
            <div className="space-y-2">
              <input
                value={newRulesetName}
                onChange={(e) => setNewRulesetName(e.target.value)}
                className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
                placeholder="Ruleset name (e.g., Mothership 1E)"
              />
              <button
                type="button"
                onClick={() => {
                  setShowNewRuleset(false);
                  setNewRulesetName('');
                }}
                className="text-xs text-gray-400 hover:text-gray-300"
              >
                Use existing instead
              </button>
            </div>
          )}
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
