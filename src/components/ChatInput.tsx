import { useState, useRef, type KeyboardEvent } from 'react';

interface Props {
  onSend: (content: string) => void;
  disabled: boolean;
}

export function ChatInput({ onSend, disabled }: Props) {
  const [text, setText] = useState('');
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const handleSend = () => {
    const trimmed = text.trim();
    if (!trimmed || disabled) return;
    onSend(trimmed);
    setText('');
    if (textareaRef.current) {
      textareaRef.current.style.height = 'auto';
    }
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  const handleInput = () => {
    const el = textareaRef.current;
    if (el) {
      el.style.height = 'auto';
      el.style.height = Math.min(el.scrollHeight, 200) + 'px';
    }
  };

  return (
    <div className="px-6 py-4 border-t border-gray-700 bg-gray-800/30">
      <div className="flex gap-3">
        <textarea
          ref={textareaRef}
          value={text}
          onChange={(e) => setText(e.target.value)}
          onKeyDown={handleKeyDown}
          onInput={handleInput}
          placeholder="Describe the scene, address players..."
          disabled={disabled}
          rows={1}
          className="flex-1 bg-gray-800 border border-gray-600 rounded-lg px-4 py-3 text-sm text-gray-100 placeholder-gray-500 resize-none focus:outline-none focus:border-indigo-500 disabled:opacity-50"
        />
        <button
          onClick={handleSend}
          disabled={disabled || !text.trim()}
          className="px-5 py-3 bg-indigo-600 hover:bg-indigo-500 disabled:bg-gray-700 disabled:text-gray-500 rounded-lg text-sm font-medium transition-colors"
        >
          Send
        </button>
      </div>
    </div>
  );
}
