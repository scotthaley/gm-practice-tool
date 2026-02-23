import { AppProvider, useAppState, useAppDispatch } from './stores/appStore';
import { Layout } from './components/Layout';
import { HomeView } from './components/HomeView';
import { ChatView } from './components/ChatView';
import { CampaignSetup } from './components/CampaignSetup';
import { PlayerSetupModal } from './components/PlayerSetup';
import { CharacterSetupModal } from './components/CharacterSetup';
import { SettingsModal } from './components/SettingsView';
import { RulesetEdit } from './components/RulesetEdit';
import { DocumentModal } from './components/DocumentModal';
import { useCampaign } from './hooks/useCampaign';

function AppContent() {
  const { currentView, documentModalOpen, editingDocument, activeCampaign } = useAppState();
  const dispatch = useAppDispatch();
  const { createDocument, updateDocument, deleteDocument } = useCampaign();

  return (
    <Layout>
      {currentView === 'home' && <HomeView />}
      {currentView === 'chat' && <ChatView />}
      {currentView === 'campaign-setup' && <CampaignSetup />}
      {currentView === 'ruleset-edit' && <RulesetEdit />}
      <PlayerSetupModal />
      <CharacterSetupModal />
      <SettingsModal />
      {documentModalOpen && activeCampaign && (
        <DocumentModal
          document={editingDocument}
          campaignId={activeCampaign.id}
          onClose={() => dispatch({ type: 'SET_DOCUMENT_MODAL_OPEN', open: false })}
          onSave={(request) =>
            editingDocument ? updateDocument(request as Parameters<typeof updateDocument>[0]) : createDocument(request as Parameters<typeof createDocument>[0])
          }
          onDelete={deleteDocument}
        />
      )}
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
