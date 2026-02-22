import type { ReactNode } from 'react';
import { Sidebar } from './Sidebar';

export function Layout({ children }: { children: ReactNode }) {
  return (
    <div className="flex h-screen bg-gray-900 text-gray-100">
      <Sidebar />
      <main className="flex-1 flex flex-col overflow-hidden">{children}</main>
    </div>
  );
}
