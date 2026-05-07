import React from "react";
import { Settings as SettingsIcon, Moon, Shield } from "lucide-react";

const SettingsPage: React.FC = () => {
  return (
    <div className="space-y-6">
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
          <span className="text-sm text-zinc-400">Dark</span>
        </div>

        <div className="p-5 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Shield className="w-5 h-5 text-zinc-400" />
            <div>
              <div className="font-medium">Elevation</div>
              <div className="text-sm text-zinc-500">Run as Administrator for tweaks</div>
            </div>
          </div>
          <span className="text-sm text-emerald-400">Granted</span>
        </div>

        <div className="p-5 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <SettingsIcon className="w-5 h-5 text-zinc-400" />
            <div>
              <div className="font-medium">Version</div>
              <div className="text-sm text-zinc-500">Current app version</div>
            </div>
          </div>
          <span className="text-sm text-zinc-400">0.1.0</span>
        </div>
      </div>

      <div className="bg-surface-elevated border border-surface-border rounded-xl p-5">
        <h3 className="font-medium mb-2">About ZingerBoost</h3>
        <p className="text-sm text-zinc-400">
          ZingerBoost is an open-source Windows optimization utility built with Rust and Tauri.
          Author: YousefMohiey. Licensed under MIT.
        </p>
      </div>
    </div>
  );
};

export default SettingsPage;
