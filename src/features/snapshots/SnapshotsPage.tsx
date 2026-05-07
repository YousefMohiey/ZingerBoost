import React from "react";
import { History } from "lucide-react";

const SnapshotsPage: React.FC = () => {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold">Snapshots</h1>
        <p className="text-zinc-400 text-sm mt-1">
          Restore points and tweak history
        </p>
      </div>

      <div className="bg-surface-elevated border border-surface-border rounded-xl p-12 text-center">
        <History className="w-12 h-12 text-zinc-600 mx-auto mb-4" />
        <h3 className="text-lg font-medium text-zinc-300">No snapshots yet</h3>
        <p className="text-sm text-zinc-500 mt-2 max-w-md mx-auto">
          Snapshots are created automatically when you apply tweaks. You can restore your system to any previous state from here.
        </p>
      </div>
    </div>
  );
};

export default SnapshotsPage;
