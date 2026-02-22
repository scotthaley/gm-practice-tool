import { AppProvider, useAppState } from './stores/appStore';
import { Layout } from './components/Layout';
import { ChatView } from './components/ChatView';
import { CampaignSetup } from './components/CampaignSetup';
import { PlayerSetup } from './components/PlayerSetup';
import { SettingsView } from './components/SettingsView';

function AppContent() {
  const { currentView } = useAppState();

  return (
    <Layout>
      {currentView === 'chat' && <ChatView />}
      {currentView === 'campaign-setup' && <CampaignSetup />}
      {currentView === 'player-setup' && <PlayerSetup />}
      {currentView === 'settings' && <SettingsView />}
    </Layout>
  );
}

function App() {
  return (
    <AppProvider>
      <AppContent />
    </AppProvider>
  );
}

export default App;
