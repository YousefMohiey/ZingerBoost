import React from "react";
import { Link, useLocation } from "react-router-dom";
import {
  LayoutDashboard,
  SlidersHorizontal,
  History,
  Settings,
  ChevronLeft,
  ChevronRight,
  Shield,
} from "lucide-react";

interface SidebarProps {
  isOpen: boolean;
  onToggle: () => void;
}

const navItems = [
  { to: "/", icon: LayoutDashboard, label: "Dashboard" },
  { to: "/tweaks", icon: SlidersHorizontal, label: "Tweaks" },
  { to: "/snapshots", icon: History, label: "Snapshots" },
];

const bottomItems = [
  { to: "/settings", icon: Settings, label: "Settings" },
];

const Sidebar: React.FC<SidebarProps> = ({ isOpen, onToggle }) => {
  const location = useLocation();

  return (
    <aside
      className={`flex flex-col bg-surface-elevated border-r border-surface-border transition-all duration-200 ${
        isOpen ? "w-64" : "w-16"
      }`}
    >
      <div className="flex items-center justify-between p-4 border-b border-surface-border">
        {isOpen && (
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-lg bg-brand-600 flex items-center justify-center">
              <Shield className="w-5 h-5 text-white" />
            </div>
            <span className="font-bold text-lg tracking-tight">ZingerBoost</span>
          </div>
        )}
        <button
          onClick={onToggle}
          className="p-1.5 rounded-md hover:bg-zinc-800 transition-colors"
        >
          {isOpen ? <ChevronLeft className="w-5 h-5" /> : <ChevronRight className="w-5 h-5" />}
        </button>
      </div>

      <nav className="flex-1 py-4 space-y-1">
        {navItems.map((item) => {
          const active = location.pathname === item.to;
          return (
            <Link
              key={item.to}
              to={item.to}
              className={`flex items-center gap-3 px-4 py-2.5 mx-2 rounded-lg transition-colors ${
                active
                  ? "bg-brand-600/20 text-brand-500"
                  : "text-zinc-400 hover:bg-zinc-800 hover:text-zinc-100"
              }`}
            >
              <item.icon className="w-5 h-5 shrink-0" />
              {isOpen && <span className="text-sm font-medium">{item.label}</span>}
            </Link>
          );
        })}
      </nav>

      <div className="py-4 border-t border-surface-border space-y-1">
        {bottomItems.map((item) => (
          <Link
            key={item.to}
            to={item.to}
            className="flex items-center gap-3 px-4 py-2.5 mx-2 rounded-lg text-zinc-400 hover:bg-zinc-800 hover:text-zinc-100 transition-colors"
          >
            <item.icon className="w-5 h-5 shrink-0" />
            {isOpen && <span className="text-sm font-medium">{item.label}</span>}
          </Link>
        ))}
        {isOpen && (
          <div className="px-4 pt-2 flex items-center gap-2 text-xs text-zinc-500">
            <Shield className="w-3.5 h-3.5 text-emerald-500" />
            <span>Admin</span>
          </div>
        )}
      </div>
    </aside>
  );
};

export default Sidebar;
