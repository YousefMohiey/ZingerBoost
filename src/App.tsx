import React, { useState } from "react";
import { Routes, Route } from "react-router-dom";
import Sidebar from "./components/ui/Sidebar";
import ToastContainer from "./components/ui/ToastContainer";
import Dashboard from "./features/dashboard/Dashboard";
import TweaksPage from "./features/tweaks/TweaksPage";
import SnapshotsPage from "./features/snapshots/SnapshotsPage";
import SettingsPage from "./features/settings/SettingsPage";
import SoftwarePage from "./features/software/SoftwarePage";

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
          <Route path="/software" element={<SoftwarePage />} />
          <Route path="/settings" element={<SettingsPage />} />
        </Routes>
      </main>
      <ToastContainer />
    </div>
  );
};

export default App;
