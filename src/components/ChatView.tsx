import { useEffect, useRef } from 'react';
import { useChat } from '../hooks/useChat';
import { useAppState } from '../stores/appStore';
import { ChatMessage } from './ChatMessage';
import { ChatInput } from './ChatInput';

export function ChatView() {
  const { messages, loading, error, sendMessage } = useChat();
  const { activeCampaign, players } = useAppState();
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  if (!activeCampaign) {
    return (
      <div className="flex-1 flex items-center justify-center text-gray-500">
        <div className="text-center">
          <p className="text-lg mb-2">No campaign selected</p>
          <p className="text-sm">Create or select a campaign to start</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex-1 flex flex-col">
      {/* Header */}
      <div className="px-6 py-3 border-b border-gray-700 bg-gray-800/50">
        <h2 className="text-lg font-semibold">{activeCampaign.name}</h2>
        <p className="text-xs text-gray-400">{activeCampaign.setting || 'No setting defined'}</p>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto px-6 py-4 space-y-4">
        {messages.length === 0 && (
          <div className="text-center text-gray-500 mt-8">
            <p>No messages yet. Set the scene for your players!</p>
          </div>
        )}
        {messages.map((msg) => (
          <ChatMessage key={msg.id} message={msg} players={players} />
        ))}
        {loading && (
          <div className="flex items-center gap-2 text-gray-400">
            <div className="animate-spin h-4 w-4 border-2 border-gray-400 border-t-transparent rounded-full" />
            <span className="text-sm">Players are responding...</span>
          </div>
        )}
        <div ref={messagesEndRef} />
      </div>

      {/* Error */}
      {error && (
        <div className="px-6 py-2 bg-red-900/30 border-t border-red-800 text-red-300 text-sm">
          {error}
        </div>
      )}

      {/* Input */}
      <ChatInput onSend={sendMessage} disabled={loading} />
    </div>
  );
}
