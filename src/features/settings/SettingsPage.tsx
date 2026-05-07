import React from "react";
import { useQuery } from "@tanstack/react-query";
import { Settings as SettingsIcon, Moon, Shield, FileText, Clock } from "lucide-react";
import { api } from "../../lib/api";

const SettingsPage: React.FC = () => {
  const { data: auditData } = useQuery({
    queryKey: ["audit"],
    queryFn: api.getAuditLog,
    refetchInterval: 5000,
  });

  const levelColors: Record<string, string> = {
    Info: "text-brand-400 bg-brand-500/10",
    Warn: "text-amber-400 bg-amber-500/10",
    Error: "text-red-400 bg-red-500/10",
    Debug: "text-zinc-400 bg-zinc-500/10",
  };

  return (
    <div className="space-y-6 max-w-5xl">
      <div>
        <h1 className="text-2xl font-bold">Settings</h1>
        <p className="text-zinc-400 text-sm mt-1">
          Configure ZingerBoost behavior
        </p>
      </div>

      <div className="bg-surface-elevated border border-surface-border rounded-xl divide-y divide-surface-border">
        <div className="p-5 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Moon className="w-5 h-5 text-zinc-400" />
            <div>
              <div className="font-medium">Theme</div>
              <div className="text-sm text-zinc-500">Dark mode is enabled by default</div>
            </div>
          </div>
          <span className="text-sm text-zinc-400 font-medium">Dark</span>
        </div>

        <div className="p-5 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Shield className="w-5 h-5 text-zinc-400" />
            <div>
              <div className="font-medium">Elevation</div>
              <div className="text-sm text-zinc-500">Run as Administrator for tweaks</div>
            </div>
          </div>
          <span className="text-sm text-emerald-400 font-medium">Granted</span>
        </div>

        <div className="p-5 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <SettingsIcon className="w-5 h-5 text-zinc-400" />
            <div>
              <div className="font-medium">Version</div>
              <div className="text-sm text-zinc-500">Current app version</div>
            </div>
          </div>
          <span className="text-sm text-zinc-400 font-medium">0.1.0</span>
        </div>
      </div>

      <div className="bg-surface-elevated border border-surface-border rounded-xl p-5">
        <h3 className="font-medium mb-2">About ZingerBoost</h3>
        <p className="text-sm text-zinc-400 leading-relaxed">
          ZingerBoost is an open-source Windows optimization utility built with Rust and Tauri.
          Author: YousefMohiey. Licensed under MIT.
        </p>
      </div>

      <div>
        <h3 className="font-medium mb-3 flex items-center gap-2">
          <FileText className="w-4 h-4 text-zinc-400" />
          Recent Activity
        </h3>
        {auditData && auditData.entries.length > 0 ? (
          <div className="bg-surface-elevated border border-surface-border rounded-xl divide-y divide-surface-border">
            {auditData.entries.slice(0, 20).map((entry, i) => (
              <div key={i} className="p-4 flex items-center gap-3">
                <span className={`text-xs px-2 py-0.5 rounded-full font-medium ${levelColors[entry.level] || "text-zinc-400 bg-zinc-500/10"}`}>
                  {entry.level}
                </span>
                <span className="text-sm text-zinc-300 flex-1">{entry.message}</span>
                <span className="text-xs text-zinc-600 flex items-center gap-1">
                  <Clock className="w-3 h-3" />
                  {new Date(entry.timestamp).toLocaleTimeString()}
                </span>
              </div>
            ))}
          </div>
        ) : (
          <div className="bg-surface-elevated border border-surface-border rounded-xl p-8 text-center text-sm text-zinc-500">
            No activity yet. Apply a tweak to see it here.
          </div>
        )}
      </div>
    </div>
  );
};

export default SettingsPage;
