export function DiceResult({ result }: { result: string }) {
  return (
    <span className="inline-flex items-center gap-1 px-2 py-1 bg-amber-900/30 border border-amber-700/50 rounded text-xs text-amber-300">
      🎲 {result}
    </span>
  );
}
