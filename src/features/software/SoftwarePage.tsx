import React, { useState } from "react";
import { useQuery, useMutation } from "@tanstack/react-query";
import { Download, Globe, Trash2, Shield, Package, AlertTriangle, Monitor } from "lucide-react";
import { motion } from "framer-motion";
import { useToastStore } from "../../store/toast";

interface SoftwarePackage {
  id: string;
  name: string;
  description: string;
  category: string;
  winget_id: string;
  website: string | null;
  free: boolean;
}

interface BloatwareData {
  bloatware: SoftwarePackage[];
  protected: string[];
}

const categoryIcons: Record<string, React.ElementType> = {
  browsers: Globe,
  media_players: Monitor,
  gaming: Package,
  utilities: Package,
  drivers: Monitor,
  communication: Globe,
  development: Package,
  cloud_storage: Globe,
};

const categoryLabels: Record<string, string> = {
  browsers: "Browsers",
  media_players: "Media Players",
  gaming: "Gaming",
  utilities: "Utilities",
  drivers: "Drivers",
  communication: "Communication",
  development: "Development",
  cloud_storage: "Cloud Storage",
  bloatware: "Bloatware",
};

const SoftwarePage: React.FC = () => {
  const [activeTab, setActiveTab] = useState<"install" | "debloat">("install");
  const [activeCategory, setActiveCategory] = useState<string>("all");
  const addToast = useToastStore((s) => s.addToast);
  const [installing, setInstalling] = useState<Set<string>>(new Set());

  const { data: catalogData } = useQuery({
    queryKey: ["software"],
    queryFn: async () => {
      return (await import("@tauri-apps/api/core").then(m => m.invoke("list_software"))) as SoftwarePackage[];
    },
  });

  const { data: bloatData } = useQuery({
    queryKey: ["bloatware"],
    queryFn: async () => {
      return (await import("@tauri-apps/api/core").then(m => m.invoke("list_bloatware"))) as BloatwareData;
    },
  });

  const handleInstall = async (packageName: string) => {
    setInstalling(prev => new Set(prev).add(packageName));
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const result = await invoke<{ success: boolean; message: string }>("install_software", {
        request: { winget_id: packageName },
      });
      addToast(result.message, result.success ? "success" : "error");
    } catch (e: any) {
      addToast(e.message || "Install failed", "error");
    } finally {
      setInstalling(prev => {
        const next = new Set(prev);
        next.delete(packageName);
        return next;
      });
    }
  };

  const handleRemoveAll = async () => {
    if (!bloatData?.bloatware) return;
    addToast("Removing bloatware... (keeping Notepad, Calculator, Store, Photos)", "info");
    for (const app of bloatData.bloatware) {
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        const results = await invoke<{ success: boolean; message: string }[]>("remove_bloatware", {
          request: { package_ids: [app.name] },
        });
        if (results.length > 0) {
          addToast(results[0].message, results[0].success ? "success" : "error");
        }
      } catch (e: any) {
        addToast(e.message || "Removal failed", "error");
      }
    }
  };

  const catalog: SoftwarePackage[] = catalogData || [];
  const categories = ["all", ...new Set(catalog.map((s) => s.category))];
  const filtered = activeCategory === "all"
    ? catalog
    : catalog.filter((s) => s.category === activeCategory);

  return (
    <div className="space-y-6 max-w-6xl">
      <div>
        <h1 className="text-2xl font-bold">Software</h1>
        <p className="text-zinc-400 text-sm mt-1">
          Install apps or remove Windows bloatware
        </p>
      </div>

      <div className="flex gap-2">
        <button
          onClick={() => setActiveTab("install")}
          className={`px-4 py-2 text-sm font-medium rounded-lg transition-all ${
            activeTab === "install"
              ? "bg-brand-600 text-white shadow-lg shadow-brand-600/20"
              : "bg-zinc-800 text-zinc-400 hover:bg-zinc-700 hover:text-zinc-200"
          }`}
        >
          <Download className="w-4 h-4 inline mr-2" />
          Install Apps
        </button>
        <button
          onClick={() => setActiveTab("debloat")}
          className={`px-4 py-2 text-sm font-medium rounded-lg transition-all ${
            activeTab === "debloat"
              ? "bg-red-600 text-white shadow-lg shadow-red-600/20"
              : "bg-zinc-800 text-zinc-400 hover:bg-zinc-700 hover:text-zinc-200"
          }`}
        >
          <Trash2 className="w-4 h-4 inline mr-2" />
          Debloat Windows
        </button>
      </div>

      {activeTab === "install" ? (
        <>
          <div className="flex gap-2 flex-wrap">
            {categories.map((cat) => (
              <button
                key={cat}
                onClick={() => setActiveCategory(cat)}
                className={`px-3 py-2 text-sm rounded-lg transition-all font-medium ${
                  activeCategory === cat
                    ? "bg-brand-600 text-white"
                    : "bg-zinc-800 text-zinc-400 hover:bg-zinc-700 hover:text-zinc-200"
                }`}
              >
                {categoryLabels[cat] || cat}
              </button>
            ))}
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {filtered.map((pkg) => {
              const Icon = categoryIcons[pkg.category] || Package;
              const isInstalling = installing.has(pkg.winget_id);
              return (
                <motion.div
                  key={pkg.id}
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  className="bg-surface-elevated border border-surface-border rounded-xl p-5 hover:border-surface-border/80 transition-colors flex flex-col"
                >
                  <div className="flex items-center gap-3 mb-3">
                    <div className="p-2 bg-brand-600/20 rounded-lg">
                      <Icon className="w-5 h-5 text-brand-500" />
                    </div>
                    <div className="flex-1 min-w-0">
                      <h3 className="font-semibold text-sm truncate">{pkg.name}</h3>
                      <span className="text-xs text-zinc-500">
                        {categoryLabels[pkg.category] || pkg.category}
                      </span>
                    </div>
                  </div>
                  <p className="text-xs text-zinc-400 mb-4 flex-1 leading-relaxed">
                    {pkg.description}
                  </p>
                  <div className="flex items-center gap-2">
                    <button
                      onClick={() => handleInstall(pkg.winget_id)}
                      disabled={isInstalling}
                      className="flex-1 px-3 py-2 bg-brand-600 hover:bg-brand-500 disabled:opacity-50 disabled:cursor-not-allowed text-white text-xs font-medium rounded-lg transition-all active:scale-95"
                    >
                      {isInstalling ? "Installing..." : `Install via Winget`}
                    </button>
                    {pkg.website && (
                      <a
                        href={pkg.website}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="p-2 bg-zinc-800 hover:bg-zinc-700 rounded-lg transition-colors"
                        title="Open website"
                      >
                        <Globe className="w-4 h-4 text-zinc-400" />
                      </a>
                    )}
                  </div>
                </motion.div>
              );
            })}
          </div>
        </>
      ) : (
        <>
          <div className="bg-amber-500/10 border border-amber-500/20 rounded-xl p-4 flex items-start gap-3 mb-4">
            <AlertTriangle className="w-5 h-5 text-amber-400 shrink-0 mt-0.5" />
            <div>
              <div className="text-sm font-medium text-amber-400">Protected apps will be kept</div>
              <p className="text-xs text-amber-400/80 mt-1">
                Notepad, Calculator, Microsoft Store, Photos, Camera, Snipping Tool, and system
                runtimes will NOT be removed.
              </p>
            </div>
          </div>

          <div className="flex items-center justify-between mb-2">
            <h2 className="text-lg font-semibold flex items-center gap-2">
              <Trash2 className="w-5 h-5 text-red-400" />
              Bloatware to Remove
            </h2>
            <button
              onClick={handleRemoveAll}
              className="px-4 py-2 bg-red-600 hover:bg-red-500 text-white text-sm font-medium rounded-lg transition-all active:scale-95"
            >
              Remove All Bloatware
            </button>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-3 mb-8">
            {bloatData?.bloatware.map((app) => (
              <div
                key={app.id}
                className="bg-surface-elevated border border-surface-border rounded-lg p-4 flex items-center justify-between hover:border-red-500/20 transition-colors"
              >
                <div className="flex items-center gap-3 min-w-0">
                  <div className="p-1.5 bg-red-500/20 rounded-lg">
                    <Trash2 className="w-4 h-4 text-red-400" />
                  </div>
                  <div className="min-w-0">
                    <div className="text-sm font-medium truncate">{app.name}</div>
                    <div className="text-xs text-zinc-500 truncate">{app.description}</div>
                  </div>
                </div>
              </div>
            ))}
          </div>

          <div className="bg-emerald-500/10 border border-emerald-500/20 rounded-xl p-4 flex items-start gap-3">
            <Shield className="w-5 h-5 text-emerald-400 shrink-0 mt-0.5" />
            <div>
              <div className="text-sm font-medium text-emerald-400">Protected System Apps</div>
              <div className="text-xs text-emerald-400/80 mt-1 leading-relaxed">
                {bloatData?.protected.map((app, i) => (
                  <span key={i} className="inline-block bg-emerald-500/10 px-2 py-0.5 rounded mr-1.5 mb-1">
                    {app.replace("Microsoft.", "")}
                  </span>
                ))}
              </div>
            </div>
          </div>
        </>
      )}
    </div>
  );
};

export default SoftwarePage;
