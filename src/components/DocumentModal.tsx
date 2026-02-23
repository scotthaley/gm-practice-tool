import { useState, useEffect } from 'react';
import type { Document, CreateDocumentRequest, UpdateDocumentRequest } from '../lib/types';

interface Props {
  document: Document | null;
  campaignId: string;
  onClose: () => void;
  onSave: (request: CreateDocumentRequest | UpdateDocumentRequest) => Promise<unknown>;
  onDelete: (documentId: string) => Promise<void>;
}

export function DocumentModal({ document, campaignId, onClose, onSave, onDelete }: Props) {
  const [name, setName] = useState('');
  const [content, setContent] = useState('');
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (document) {
      setName(document.name);
      setContent(document.content);
    } else {
      setName('');
      setContent('');
    }
  }, [document]);

  const handleSave = async () => {
    if (!name.trim() || saving) return;
    setSaving(true);
    try {
      if (document) {
        await onSave({ id: document.id, name: name.trim(), content } as UpdateDocumentRequest);
      } else {
        await onSave({ campaign_id: campaignId, name: name.trim(), content } as CreateDocumentRequest);
      }
      onClose();
    } finally {
      setSaving(false);
    }
  };

  const handleDelete = async () => {
    if (!document || saving) return;
    setSaving(true);
    try {
      await onDelete(document.id);
      onClose();
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60" onClick={onClose}>
      <div
        className="bg-gray-800 border border-gray-700 rounded-lg shadow-xl w-full max-w-lg mx-4"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between p-4 border-b border-gray-700">
          <h2 className="text-lg font-semibold">
            {document ? 'Edit Document' : 'New Document'}
          </h2>
          <button onClick={onClose} className="text-gray-400 hover:text-gray-200 text-xl leading-none">
            &times;
          </button>
        </div>

        <div className="p-4 space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">Name</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g. World Lore, Magic System"
              className="w-full bg-gray-900 border border-gray-600 rounded px-3 py-2 text-sm text-gray-100 placeholder-gray-500 focus:outline-none focus:border-indigo-500"
            />
            <p className="text-xs text-gray-500 mt-1">Reference in chat with @{name || 'Name'}</p>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">Content</label>
            <textarea
              value={content}
              onChange={(e) => setContent(e.target.value)}
              placeholder="Document content that will be injected when referenced..."
              rows={10}
              className="w-full bg-gray-900 border border-gray-600 rounded px-3 py-2 text-sm text-gray-100 placeholder-gray-500 resize-y focus:outline-none focus:border-indigo-500"
            />
          </div>
        </div>

        <div className="flex items-center justify-between p-4 border-t border-gray-700">
          <div>
            {document && (
              <button
                onClick={handleDelete}
                disabled={saving}
                className="px-3 py-2 text-sm text-red-400 hover:text-red-300 hover:bg-red-900/30 rounded transition-colors disabled:opacity-50"
              >
                Delete
              </button>
            )}
          </div>
          <div className="flex gap-2">
            <button
              onClick={onClose}
              className="px-4 py-2 text-sm text-gray-400 hover:text-gray-200 transition-colors"
            >
              Cancel
            </button>
            <button
              onClick={handleSave}
              disabled={!name.trim() || saving}
              className="px-4 py-2 bg-indigo-600 hover:bg-indigo-500 disabled:bg-gray-700 disabled:text-gray-500 rounded text-sm font-medium transition-colors"
            >
              {saving ? 'Saving...' : 'Save'}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
