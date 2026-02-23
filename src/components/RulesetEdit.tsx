import { useState, useEffect, type FormEvent } from 'react';
import { useAppState } from '../stores/appStore';
import { useRuleset } from '../hooks/useRuleset';

export function RulesetEdit() {
  const { rulesets } = useAppState();
  const { createRuleset, updateRuleset, deleteRuleset } = useRuleset();

  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [name, setName] = useState('');
  const [content, setContent] = useState('');
  const [submitting, setSubmitting] = useState(false);

  const selected = rulesets.find((r) => r.id === selectedId) ?? null;

  useEffect(() => {
    if (selected) {
      setName(selected.name);
      setContent(selected.content);
    }
  }, [selected]);

  const handleNew = () => {
    setSelectedId(null);
    setName('');
    setContent('');
  };

  const handleSelect = (id: string) => {
    setSelectedId(id);
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;
    setSubmitting(true);
    try {
      if (selectedId) {
        await updateRuleset(selectedId, name.trim(), content);
      } else {
        const rs = await createRuleset(name.trim(), content);
        setSelectedId(rs.id);
      }
    } finally {
      setSubmitting(false);
    }
  };

  const handleDelete = async () => {
    if (!selectedId) return;
    setSubmitting(true);
    try {
      await deleteRuleset(selectedId);
      handleNew();
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className="flex-1 flex h-full">
      {/* Ruleset list */}
      <div className="w-64 border-r border-gray-700 flex flex-col">
        <div className="p-3 border-b border-gray-700 flex items-center justify-between">
          <h2 className="text-sm font-semibold text-gray-300">Rulesets</h2>
          <button
            onClick={handleNew}
            className="text-xs text-indigo-400 hover:text-indigo-300"
          >
            + New
          </button>
        </div>
        <div className="flex-1 overflow-y-auto">
          {rulesets.map((r) => (
            <button
              key={r.id}
              onClick={() => handleSelect(r.id)}
              className={`w-full text-left px-3 py-2 text-sm border-b border-gray-700/50 transition-colors ${
                selectedId === r.id
                  ? 'bg-gray-700 text-gray-100'
                  : 'text-gray-400 hover:bg-gray-700/50 hover:text-gray-200'
              }`}
            >
              {r.name}
            </button>
          ))}
          {rulesets.length === 0 && (
            <p className="text-xs text-gray-500 p-3">No rulesets yet</p>
          )}
        </div>
      </div>

      {/* Editor */}
      <div className="flex-1 flex items-start justify-center p-8 overflow-y-auto">
        <form onSubmit={handleSubmit} className="w-full max-w-2xl space-y-4">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-bold">
              {selectedId ? 'Edit Ruleset' : 'New Ruleset'}
            </h2>
            {selectedId && (
              <button
                type="button"
                onClick={handleDelete}
                disabled={submitting}
                className="text-xs text-red-400 hover:text-red-300 disabled:text-gray-600"
              >
                Delete
              </button>
            )}
          </div>

          <div>
            <label className="block text-sm text-gray-400 mb-1">Name *</label>
            <input
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
              placeholder="Mothership 1E"
            />
          </div>

          <div>
            <label className="block text-sm text-gray-400 mb-1">Rules Content</label>
            <textarea
              value={content}
              onChange={(e) => setContent(e.target.value)}
              className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500 h-96 resize-y font-mono"
              placeholder="Paste or write game rules here. Players will reference these rules during play..."
            />
          </div>

          <button
            type="submit"
            disabled={!name.trim() || submitting}
            className="w-full py-2 bg-indigo-600 hover:bg-indigo-500 disabled:bg-gray-700 disabled:text-gray-500 rounded text-sm font-medium transition-colors"
          >
            {submitting ? 'Saving...' : selectedId ? 'Save Changes' : 'Create Ruleset'}
          </button>
        </form>
      </div>
    </div>
  );
}
