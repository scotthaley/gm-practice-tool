import type { Message, Player } from '../lib/types';
import { DiceResult } from './DiceResult';

interface Props {
  message: Message;
  players: Player[];
}

export function ChatMessage({ message, players }: Props) {
  const isGm = message.sender_type === 'gm';
  const isSystem = message.sender_type === 'system';
  const player = players.find((p) => p.id === message.sender_id);
  const color = player?.color || '#6366f1';

  const diceResults = message.metadata?.tool_calls?.filter((tc) => tc.tool === 'roll_dice') || [];

  if (isSystem) {
    return (
      <div className="text-center text-xs text-gray-500 py-1">
        {message.content}
      </div>
    );
  }

  return (
    <div className={`flex ${isGm ? 'justify-end' : 'justify-start'}`}>
      <div
        className={`max-w-[75%] rounded-lg px-4 py-3 ${
          isGm
            ? 'bg-indigo-600/20 border border-indigo-500/30'
            : 'bg-gray-800 border border-gray-700'
        }`}
        style={!isGm ? { borderLeftColor: color, borderLeftWidth: '3px' } : undefined}
      >
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
      </div>
    </div>
  );
}
