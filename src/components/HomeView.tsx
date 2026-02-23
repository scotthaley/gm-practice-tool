import { useCampaign } from '../hooks/useCampaign';
import { useAppDispatch } from '../stores/appStore';

export function HomeView() {
  const { campaigns, selectCampaign } = useCampaign();
  const dispatch = useAppDispatch();

  return (
    <div className="flex-1 flex items-center justify-center">
      <div className="w-full max-w-md">
        <h2 className="text-xl font-bold mb-6 text-center">Select a Campaign</h2>

        <div className="space-y-2 mb-6">
          {campaigns.map((c) => (
            <button
              key={c.id}
              onClick={() => selectCampaign(c.id)}
              className="w-full text-left px-4 py-3 rounded-lg bg-gray-800 hover:bg-gray-700 transition-colors"
            >
              <div className="font-medium text-gray-100">{c.name}</div>
              {c.description && (
                <div className="text-sm text-gray-400 mt-1 line-clamp-2">{c.description}</div>
              )}
            </button>
          ))}
          {campaigns.length === 0 && (
            <p className="text-center text-gray-500">No campaigns yet. Create one to get started!</p>
          )}
        </div>

        <button
          onClick={() => dispatch({ type: 'SET_VIEW', view: 'campaign-setup' })}
          className="w-full px-4 py-3 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-sm font-medium transition-colors"
        >
          + New Campaign
        </button>
      </div>
    </div>
  );
}
