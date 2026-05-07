import React from "react";
import { useQuery } from "@tanstack/react-query";
import { History, Calendar, Layers } from "lucide-react";
import { api } from "../../lib/api";

const SnapshotsPage: React.FC = () => {
  const { data: snapshots, isLoading } = useQuery({
    queryKey: ["snapshots"],
    queryFn: api.listSnapshots,
  });

  return (
    <div className="space-y-6 max-w-5xl">
      <div>
        <h1 className="text-2xl font-bold">Snapshots</h1>
        <p className="text-zinc-400 text-sm mt-1">
          Restore points and tweak history
        </p>
      </div>

      {isLoading ? (
        <div className="flex items-center justify-center py-12">
          <div className="w-8 h-8 border-2 border-brand-600 border-t-transparent rounded-full animate-spin" />
        </div>
      ) : snapshots && snapshots.length > 0 ? (
        <div className="space-y-3">
          {snapshots.map((snapshot) => (
            <div
              key={snapshot.id}
              className="bg-surface-elevated border border-surface-border rounded-xl p-5 hover:border-surface-border/80 transition-colors"
            >
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="p-2 bg-brand-600/20 rounded-lg">
                    <History className="w-5 h-5 text-brand-500" />
                  </div>
                  <div>
                    <div className="font-medium text-sm">{snapshot.description}</div>
                    <div className="flex items-center gap-3 mt-1 text-xs text-zinc-500">
                      <span className="flex items-center gap-1">
                        <Calendar className="w-3 h-3" />
                        {new Date(snapshot.created_at).toLocaleString()}
                      </span>
                      <span className="flex items-center gap-1">
                        <Layers className="w-3 h-3" />
                        {snapshot.tweak_records.length} tweak{snapshot.tweak_records.length !== 1 ? "s" : ""}
                      </span>
                    </div>
                  </div>
                </div>
                <button className="px-3 py-1.5 text-xs font-medium bg-zinc-800 hover:bg-zinc-700 text-zinc-300 rounded-lg transition-colors">
                  Restore
                </button>
              </div>
            </div>
          ))}
        </div>
      ) : (
        <div className="bg-surface-elevated border border-surface-border rounded-xl p-12 text-center">
          <History className="w-12 h-12 text-zinc-600 mx-auto mb-4" />
          <h3 className="text-lg font-medium text-zinc-300">No snapshots yet</h3>
          <p className="text-sm text-zinc-500 mt-2 max-w-md mx-auto">
            Snapshots are created automatically when you apply tweaks. You can restore your system to any previous state from here.
          </p>
        </div>
      )}
    </div>
  );
};

export default SnapshotsPage;
