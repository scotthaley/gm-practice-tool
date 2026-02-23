import { AppProvider, useAppState } from './stores/appStore';
import { Layout } from './components/Layout';
import { HomeView } from './components/HomeView';
import { ChatView } from './components/ChatView';
import { CampaignSetup } from './components/CampaignSetup';
import { PlayerSetupModal } from './components/PlayerSetup';
import { CharacterSetupModal } from './components/CharacterSetup';
import { SettingsModal } from './components/SettingsView';
import { RulesetEdit } from './components/RulesetEdit';

function AppContent() {
  const { currentView } = useAppState();

  return (
    <Layout>
      {currentView === 'home' && <HomeView />}
      {currentView === 'chat' && <ChatView />}
      {currentView === 'campaign-setup' && <CampaignSetup />}
      {currentView === 'ruleset-edit' && <RulesetEdit />}
      <PlayerSetupModal />
      <CharacterSetupModal />
      <SettingsModal />
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
