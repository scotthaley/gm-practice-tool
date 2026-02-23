import type { ToolCall } from '../lib/types';

const TOOL_CONFIG: Record<string, { icon: string; label: string; field: string }> = {
  store_memory: { icon: '\u{1F4AD}', label: 'Committed to memory', field: 'content' },
  add_campaign_log: { icon: '\u{1F4DD}', label: 'Log entry added', field: 'summary' },
  create_character: { icon: '\u{1F3AD}', label: 'Character created', field: 'name' },
  update_character: { icon: '\u{1F4DD}', label: 'Character updated', field: 'character_name' },
};

export function ToolCallNotification({ toolCall }: { toolCall: ToolCall }) {
  const config = TOOL_CONFIG[toolCall.tool];
  if (!config) return null;

  const detail = (toolCall.input[config.field] as string) || toolCall.result;

  return (
    <span className="inline-flex items-center gap-1 px-2 py-1 bg-gray-700/30 border border-gray-600/50 rounded text-xs text-gray-400">
      {config.icon} {config.label}: {detail}
    </span>
  );
}
