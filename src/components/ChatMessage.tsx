import { useState } from 'react';
import type { Message, Player } from '../lib/types';
import { DiceResult } from './DiceResult';
import { ToolCallNotification } from './ToolCallNotification';
import { PromptDebugModal } from './PromptDebugModal';
import * as api from '../lib/api';
import { useAppDispatch } from '../stores/appStore';

interface Props {
  message: Message;
  players: Player[];
}

export function ChatMessage({ message, players }: Props) {
  const dispatch = useAppDispatch();
  const [showPrompt, setShowPrompt] = useState(false);
  const isGm = message.sender_type === 'gm';
  const isSystem = message.sender_type === 'system';
  const player = players.find((p) => p.id === message.sender_id);
  const color = player?.color || '#6366f1';

  const diceResults = message.metadata?.tool_calls?.filter((tc) => tc.tool === 'roll_dice') || [];
  const notificationTools = ['store_memory', 'add_campaign_log', 'create_character', 'update_character'];
  const toolNotifications = message.metadata?.tool_calls?.filter((tc) => notificationTools.includes(tc.tool)) || [];
  const ruleUpdates = message.metadata?.rule_updates || [];

  const handleDelete = async () => {
    await api.deleteMessage(message.id);
    dispatch({ type: 'DELETE_MESSAGE', messageId: message.id });
  };

  if (isSystem) {
    return (
      <div className="text-center text-xs text-gray-500 py-1">
        {message.content}
      </div>
    );
  }

  return (
    <div className={`group flex ${isGm ? 'justify-end' : 'justify-start'}`}>
      <div
        className={`relative max-w-[75%] rounded-lg px-4 py-3 ${
          isGm
            ? 'bg-indigo-600/20 border border-indigo-500/30'
            : 'bg-gray-800 border border-gray-700'
        }`}
        style={!isGm ? { borderLeftColor: color, borderLeftWidth: '3px' } : undefined}
      >
        <div className="absolute top-1 right-1 hidden group-hover:flex items-center gap-1">
          {!isGm && !isSystem && (
            <button
              onClick={() => setShowPrompt(true)}
              className="text-gray-500 hover:text-blue-400 text-xs px-1.5 py-0.5 rounded hover:bg-gray-700/50 transition-colors"
              title="View prompt"
            >
              {"{ }"}
            </button>
          )}
          <button
            onClick={handleDelete}
            className="text-gray-500 hover:text-red-400 text-xs px-1.5 py-0.5 rounded hover:bg-gray-700/50 transition-colors"
            title="Delete message"
          >
            &times;
          </button>
        </div>
        <div className="flex items-center gap-2 mb-1">
          <span
            className="text-xs font-semibold"
            style={{ color: isGm ? '#818cf8' : color }}
          >
            {message.sender_name}
          </span>
          <span className="text-xs text-gray-500">
            {new Date(message.timestamp).toLocaleTimeString()}
          </span>
        </div>
        <div className="text-sm text-gray-200 whitespace-pre-wrap">{message.content}</div>
        {diceResults.length > 0 && (
          <div className="mt-2 space-y-1">
            {diceResults.map((dr, i) => (
              <DiceResult key={i} result={dr.result} />
            ))}
          </div>
        )}
        {toolNotifications.length > 0 && (
          <div className="mt-2 space-y-1">
            {toolNotifications.map((tc, i) => (
              <ToolCallNotification key={i} toolCall={tc} />
            ))}
          </div>
        )}
        {ruleUpdates.length > 0 && (
          <div className="mt-2 space-y-1">
            {ruleUpdates.map((rule, i) => (
              <span key={i} className="inline-flex items-center gap-1 px-2 py-1 bg-amber-900/20 border border-amber-600/30 rounded text-xs text-amber-400">
                {'\u{1F4DC}'} Rule updated: {rule}
              </span>
            ))}
          </div>
        )}
      </div>
      {showPrompt && (
        <PromptDebugModal
          messageId={message.id}
          onClose={() => setShowPrompt(false)}
        />
      )}
    </div>
  );
}
