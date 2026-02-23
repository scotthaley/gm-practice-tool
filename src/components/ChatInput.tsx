import { useState, useRef, useMemo, type KeyboardEvent } from 'react';
import type { Document } from '../lib/types';

interface Props {
  onSend: (content: string) => void;
  disabled: boolean;
  documents: Document[];
}

export function ChatInput({ onSend, disabled, documents }: Props) {
  const [text, setText] = useState('');
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const [showMentions, setShowMentions] = useState(false);
  const [mentionQuery, setMentionQuery] = useState('');
  const [mentionStartIndex, setMentionStartIndex] = useState(0);
  const [selectedMentionIndex, setSelectedMentionIndex] = useState(0);

  const filteredDocs = useMemo(() => {
    if (!showMentions) return [];
    const q = mentionQuery.toLowerCase();
    return documents.filter((d) => d.name.toLowerCase().includes(q));
  }, [showMentions, mentionQuery, documents]);

  const handleSend = () => {
    const trimmed = text.trim();
    if (!trimmed || disabled) return;
    onSend(trimmed);
    setText('');
    setShowMentions(false);
    if (textareaRef.current) {
      textareaRef.current.style.height = 'auto';
    }
  };

  const insertMention = (docName: string) => {
    const before = text.slice(0, mentionStartIndex);
    const after = text.slice(textareaRef.current?.selectionStart ?? text.length);
    const newText = before + '@' + docName + ' ' + after;
    setText(newText);
    setShowMentions(false);
    // Focus back on textarea
    setTimeout(() => {
      const el = textareaRef.current;
      if (el) {
        const cursorPos = before.length + docName.length + 2; // @name + space
        el.selectionStart = cursorPos;
        el.selectionEnd = cursorPos;
        el.focus();
      }
    }, 0);
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (showMentions && filteredDocs.length > 0) {
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        setSelectedMentionIndex((i) => Math.min(i + 1, filteredDocs.length - 1));
        return;
      }
      if (e.key === 'ArrowUp') {
        e.preventDefault();
        setSelectedMentionIndex((i) => Math.max(i - 1, 0));
        return;
      }
      if (e.key === 'Tab' || e.key === 'Enter') {
        e.preventDefault();
        insertMention(filteredDocs[selectedMentionIndex].name);
        return;
      }
      if (e.key === 'Escape') {
        e.preventDefault();
        setShowMentions(false);
        return;
      }
    }

    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  const handleChange = (value: string) => {
    setText(value);

    const el = textareaRef.current;
    if (!el) return;
    const cursorPos = el.selectionStart;

    // Look backwards from cursor for an @ trigger
    const textBefore = value.slice(0, cursorPos);
    const atIndex = textBefore.lastIndexOf('@');

    if (atIndex >= 0) {
      // @ must be at start or preceded by whitespace
      const charBefore = atIndex > 0 ? textBefore[atIndex - 1] : ' ';
      if (/\s/.test(charBefore) || atIndex === 0) {
        const query = textBefore.slice(atIndex + 1);
        // Only show if no space in query (single-word mention trigger)
        if (!/\s/.test(query)) {
          setMentionQuery(query);
          setMentionStartIndex(atIndex);
          setShowMentions(true);
          setSelectedMentionIndex(0);
          return;
        }
      }
    }

    setShowMentions(false);
  };

  const handleInput = () => {
    const el = textareaRef.current;
    if (el) {
      el.style.height = 'auto';
      el.style.height = Math.min(el.scrollHeight, 200) + 'px';
    }
  };

  return (
    <div className="px-6 py-4 border-t border-gray-700 bg-gray-800/30 relative">
      {/* Mention autocomplete dropdown */}
      {showMentions && filteredDocs.length > 0 && (
        <div className="absolute bottom-full left-6 right-6 mb-1 bg-gray-800 border border-gray-600 rounded-lg shadow-xl max-h-48 overflow-y-auto z-10">
          {filteredDocs.map((doc, i) => (
            <button
              key={doc.id}
              onMouseDown={(e) => {
                e.preventDefault();
                insertMention(doc.name);
              }}
              className={`w-full text-left px-3 py-2 text-sm transition-colors ${
                i === selectedMentionIndex
                  ? 'bg-indigo-600/30 text-gray-100'
                  : 'text-gray-300 hover:bg-gray-700'
              }`}
            >
              <span className="font-medium">@{doc.name}</span>
              {doc.content && (
                <span className="text-xs text-gray-500 ml-2 truncate">
                  {doc.content.slice(0, 60)}
                  {doc.content.length > 60 ? '...' : ''}
                </span>
              )}
            </button>
          ))}
        </div>
      )}

      <div className="flex gap-3">
        <textarea
          ref={textareaRef}
          value={text}
          onChange={(e) => handleChange(e.target.value)}
          onKeyDown={handleKeyDown}
          onInput={handleInput}
          placeholder="Describe the scene, address players... (use @ to reference documents)"
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
