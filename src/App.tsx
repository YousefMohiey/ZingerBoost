import React, { useState } from "react";
import { Routes, Route } from "react-router-dom";
import Sidebar from "./components/ui/Sidebar";
import Dashboard from "./features/dashboard/Dashboard";
import TweaksPage from "./features/tweaks/TweaksPage";
import SnapshotsPage from "./features/snapshots/SnapshotsPage";
import SettingsPage from "./features/settings/SettingsPage";

const App: React.FC = () => {
  const [sidebarOpen, setSidebarOpen] = useState(true);

  return (
    <div className="flex h-screen bg-surface text-zinc-100 overflow-hidden">
      <Sidebar isOpen={sidebarOpen} onToggle={() => setSidebarOpen(!sidebarOpen)} />
      <main className="flex-1 overflow-y-auto p-6">
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/tweaks" element={<TweaksPage />} />
          <Route path="/snapshots" element={<SnapshotsPage />} />
          <Route path="/settings" element={<SettingsPage />} />
        </Routes>
      </main>
    </div>
  );
};

export default App;
