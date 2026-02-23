import { useState, useEffect, type FormEvent } from 'react';
import * as api from '../lib/api';
import type { AppConfig } from '../lib/types';
import { useAppState, useAppDispatch } from '../stores/appStore';

export function SettingsModal() {
  const { settingsOpen } = useAppState();
  const dispatch = useAppDispatch();
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    if (settingsOpen) {
      api.getConfig().then(setConfig).catch(console.error);
    }
  }, [settingsOpen]);

  if (!settingsOpen) return null;

  const close = () => dispatch({ type: 'SET_SETTINGS_OPEN', open: false });

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!config) return;
    setSaving(true);
    try {
      await api.updateConfig(config);
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (err) {
      console.error(err);
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div className="absolute inset-0 bg-black/60" onClick={close} />
      <div className="relative bg-gray-900 border border-gray-700 rounded-lg shadow-xl w-full max-w-lg max-h-[80vh] overflow-y-auto p-6">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-xl font-bold">Settings</h2>
          <button
            onClick={close}
            className="text-gray-400 hover:text-gray-200 text-lg"
          >
            &times;
          </button>
        </div>

        {!config ? (
          <div className="flex items-center justify-center py-8 text-gray-500">Loading...</div>
        ) : (
          <form onSubmit={handleSubmit} className="space-y-6">
            <div>
              <h3 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-3">API</h3>
              <div>
                <label className="block text-sm text-gray-400 mb-1">Groq API Key</label>
                <input
                  type="password"
                  value={config.api.groq_api_key}
                  onChange={(e) =>
                    setConfig({ ...config, api: { ...config.api, groq_api_key: e.target.value } })
                  }
                  className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
                  placeholder="gsk_..."
                />
              </div>
            </div>

            <div>
              <h3 className="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-3">Models</h3>
              <div className="space-y-3">
                <div>
                  <label className="block text-sm text-gray-400 mb-1">Router Model</label>
                  <input
                    value={config.models.router}
                    onChange={(e) =>
                      setConfig({ ...config, models: { ...config.models, router: e.target.value } })
                    }
                    className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
                  />
                </div>
                <div>
                  <label className="block text-sm text-gray-400 mb-1">Player Model</label>
                  <input
                    value={config.models.player}
                    onChange={(e) =>
                      setConfig({ ...config, models: { ...config.models, player: e.target.value } })
                    }
                    className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
                  />
                </div>
                <div>
                  <label className="block text-sm text-gray-400 mb-1">
                    Temperature ({config.models.parameters.temperature})
                  </label>
                  <input
                    type="range"
                    min={0}
                    max={1}
                    step={0.1}
                    value={config.models.parameters.temperature}
                    onChange={(e) =>
                      setConfig({
                        ...config,
                        models: {
                          ...config.models,
                          parameters: {
                            ...config.models.parameters,
                            temperature: parseFloat(e.target.value),
                          },
                        },
                      })
                    }
                    className="w-full"
                  />
                </div>
                <div>
                  <label className="block text-sm text-gray-400 mb-1">Player Max Tokens</label>
                  <input
                    type="number"
                    value={config.models.parameters.player_max_tokens}
                    onChange={(e) =>
                      setConfig({
                        ...config,
                        models: {
                          ...config.models,
                          parameters: {
                            ...config.models.parameters,
                            player_max_tokens: parseInt(e.target.value) || 2000,
                          },
                        },
                      })
                    }
                    className="w-full bg-gray-800 border border-gray-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
                  />
                </div>
              </div>
            </div>

            <div className="flex gap-3">
              <button
                type="submit"
                disabled={saving}
                className="px-6 py-2 bg-indigo-600 hover:bg-indigo-500 disabled:bg-gray-700 rounded text-sm font-medium transition-colors"
              >
                {saving ? 'Saving...' : saved ? 'Saved!' : 'Save Settings'}
              </button>
              <button
                type="button"
                onClick={close}
                className="px-6 py-2 bg-gray-700 hover:bg-gray-600 rounded text-sm font-medium transition-colors"
              >
                Close
              </button>
            </div>
          </form>
        )}
      </div>
    </div>
  );
}
