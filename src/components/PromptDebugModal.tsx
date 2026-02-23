import { useState, useEffect } from 'react';
import * as api from '../lib/api';
import type { MessagePromptData } from '../lib/types';

interface Props {
  messageId: string;
  onClose: () => void;
}

export function PromptDebugModal({ messageId, onClose }: Props) {
  const [data, setData] = useState<MessagePromptData | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    api.getMessagePrompt(messageId).then((result) => {
      setData(result);
      setLoading(false);
    }).catch(() => {
      setLoading(false);
    });
  }, [messageId]);

  const formatJson = (raw: string): string => {
    if (!raw) return '';
    try {
      return JSON.stringify(JSON.parse(raw), null, 2);
    } catch {
      return raw;
    }
  };

  return (
    <div
      className="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
      onClick={onClose}
    >
      <div
        className="bg-gray-900 border border-gray-700 rounded-lg max-w-4xl w-full mx-4 max-h-[80vh] flex flex-col"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between px-4 py-3 border-b border-gray-700">
          <h2 className="text-sm font-semibold text-gray-200">Prompt Debug</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-200 text-lg leading-none"
          >
            &times;
          </button>
        </div>
        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          {loading ? (
            <div className="text-center text-gray-400 py-8">Loading...</div>
          ) : !data || (!data.prompt_data && !data.llm_response) ? (
            <div className="text-center text-gray-500 py-8">
              No prompt data stored for this message
            </div>
          ) : (
            <>
              <div>
                <h3 className="text-xs font-semibold text-gray-400 uppercase mb-2">Prompt Sent</h3>
                {data.prompt_data ? (
                  <pre className="text-xs text-gray-300 bg-gray-800 rounded p-3 overflow-x-auto max-h-[35vh] overflow-y-auto whitespace-pre-wrap">
                    {formatJson(data.prompt_data)}
                  </pre>
                ) : (
                  <p className="text-sm text-gray-500">No prompt data</p>
                )}
              </div>
              <div>
                <h3 className="text-xs font-semibold text-gray-400 uppercase mb-2">LLM Response</h3>
                {data.llm_response ? (
                  <pre className="text-xs text-gray-300 bg-gray-800 rounded p-3 overflow-x-auto max-h-[35vh] overflow-y-auto whitespace-pre-wrap">
                    {formatJson(data.llm_response)}
                  </pre>
                ) : (
                  <p className="text-sm text-gray-500">No response data</p>
                )}
              </div>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
