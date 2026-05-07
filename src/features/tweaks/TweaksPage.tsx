import React, { useState } from "react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { AlertTriangle, Check, ChevronDown, ChevronUp, Info, RotateCcw, Shield, Zap } from "lucide-react";
import { api, TweakMetadata } from "../../lib/api";
import { useToastStore } from "../../store/toast";

const riskColors: Record<string, string> = {
  Safe: "bg-emerald-500/20 text-emerald-400 border-emerald-500/30",
  Moderate: "bg-amber-500/20 text-amber-400 border-amber-500/30",
  Advanced: "bg-red-500/20 text-red-400 border-red-500/30",
};

const categoryLabels: Record<string, string> = {
  visual: "Visual",
  privacy: "Privacy",
  performance: "Performance",
  gaming: "Gaming",
  debloat: "Debloat",
  network: "Network",
  startup: "Startup",
};

const categoryIcons: Record<string, string> = {
  visual: "🎨",
  privacy: "🔒",
  performance: "⚡",
  gaming: "🎮",
  debloat: "🗑️",
  network: "🌐",
  startup: "🚀",
};

const TweakCard: React.FC<{
  tweak: TweakMetadata;
  onApply: (id: string) => void;
  onRevert: (id: string) => void;
  isApplying: boolean;
  isReverting: boolean;
}> = ({ tweak, onApply, onRevert, isApplying, isReverting }) => {
  const [expanded, setExpanded] = useState(false);
  const { data: explanation } = useQuery({
    queryKey: ["explanation", tweak.id],
    queryFn: () => api.getTweakExplanation(tweak.id),
    enabled: expanded,
  });

  return (
    <div className="bg-surface-elevated border border-surface-border rounded-xl overflow-hidden hover:border-surface-border/80 transition-colors">
      <div className="p-5">
        <div className="flex items-start justify-between gap-4">
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 mb-1 flex-wrap">
              <span className="text-lg">{categoryIcons[tweak.category] || "🔧"}</span>
              <h3 className="font-semibold text-base">{tweak.name}</h3>
              <span
                className={`text-xs px-2 py-0.5 rounded-full border font-medium ${
                  riskColors[tweak.risk] || "bg-zinc-700 text-zinc-300"
                }`}
              >
                {tweak.risk}
              </span>
              {tweak.requires_reboot && (
                <span className="text-xs px-2 py-0.5 rounded-full bg-blue-500/20 text-blue-400 border border-blue-500/30 font-medium">
                  Reboot
                </span>
              )}
            </div>
            <p className="text-sm text-zinc-400 leading-relaxed">{tweak.description}</p>
            <div className="flex items-center gap-3 mt-3 flex-wrap">
              <span className="text-xs text-zinc-500 bg-zinc-800/80 px-2.5 py-1 rounded-md font-medium">
                {categoryLabels[tweak.category] || tweak.category}
              </span>
              {tweak.requires_admin && (
                <span className="flex items-center gap-1 text-xs text-zinc-500 bg-zinc-800/80 px-2 py-1 rounded-md">
                  <Shield className="w-3 h-3" /> Admin
                </span>
              )}
              {tweak.affected_keys.length > 0 && (
                <span className="text-xs text-zinc-600">
                  {tweak.affected_keys.length} registry key{tweak.affected_keys.length > 1 ? "s" : ""}
                </span>
              )}
            </div>
          </div>
          <div className="flex items-center gap-2 shrink-0">
            <button
              onClick={() => onApply(tweak.id)}
              disabled={isApplying}
              className="flex items-center gap-2 px-4 py-2 bg-brand-600 hover:bg-brand-500 disabled:opacity-50 disabled:cursor-not-allowed text-white text-sm font-medium rounded-lg transition-all active:scale-95"
            >
              <Zap className="w-4 h-4" />
              {isApplying ? "Applying..." : "Apply"}
            </button>
            <button
              onClick={() => onRevert(tweak.id)}
              disabled={isReverting}
              className="p-2 bg-zinc-800 hover:bg-zinc-700 disabled:opacity-50 disabled:cursor-not-allowed text-zinc-200 rounded-lg transition-colors active:scale-95"
              title="Revert"
            >
              <RotateCcw className="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>

      <button
        onClick={() => setExpanded(!expanded)}
        className="w-full flex items-center justify-center gap-1 py-2.5 text-xs text-zinc-500 hover:text-zinc-300 hover:bg-zinc-800/50 transition-colors border-t border-surface-border"
      >
        {expanded ? (
          <>
            <ChevronUp className="w-4 h-4" /> Show less
          </>
        ) : (
          <>
            <ChevronDown className="w-4 h-4" /> Show details
          </>
        )}
      </button>

      {expanded && explanation && (
        <div className="px-5 pb-5 pt-3 border-t border-surface-border space-y-3 bg-zinc-900/30">
          <div className="flex gap-3">
            <Info className="w-4 h-4 text-brand-500 mt-0.5 shrink-0" />
            <div>
              <div className="text-sm font-medium mb-1 text-zinc-200">What it does</div>
              <div className="text-sm text-zinc-400 leading-relaxed">{explanation.what_it_does}</div>
            </div>
          </div>
          <div className="flex gap-3">
            <Check className="w-4 h-4 text-emerald-500 mt-0.5 shrink-0" />
            <div>
              <div className="text-sm font-medium mb-1 text-zinc-200">Why it helps</div>
              <div className="text-sm text-zinc-400 leading-relaxed">{explanation.why_it_helps}</div>
            </div>
          </div>
          {explanation.potential_risks && (
            <div className="flex gap-3">
              <AlertTriangle className="w-4 h-4 text-amber-500 mt-0.5 shrink-0" />
              <div>
                <div className="text-sm font-medium mb-1 text-zinc-200">Potential risks</div>
                <div className="text-sm text-zinc-400 leading-relaxed">{explanation.potential_risks}</div>
              </div>
            </div>
          )}
          <div className="flex gap-3">
            <RotateCcw className="w-4 h-4 text-zinc-500 mt-0.5 shrink-0" />
            <div>
              <div className="text-sm font-medium mb-1 text-zinc-200">How to revert</div>
              <div className="text-sm text-zinc-400 leading-relaxed">{explanation.how_to_revert}</div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

const TweaksPage: React.FC = () => {
  const queryClient = useQueryClient();
  const addToast = useToastStore((s) => s.addToast);
  const [filter, setFilter] = useState<string>("all");
  const [search, setSearch] = useState("");

  const { data, isLoading } = useQuery({
    queryKey: ["tweaks"],
    queryFn: api.listTweaks,
  });

  const applyMutation = useMutation({
    mutationFn: api.applyTweak,
    onSuccess: (result) => {
      addToast(result.message, "success");
      queryClient.invalidateQueries({ queryKey: ["tweaks"] });
      queryClient.invalidateQueries({ queryKey: ["snapshots"] });
      queryClient.invalidateQueries({ queryKey: ["audit"] });
    },
    onError: (error: any) => {
      addToast(error.message || "Failed to apply tweak", "error");
    },
  });

  const revertMutation = useMutation({
    mutationFn: api.revertTweak,
    onSuccess: (result) => {
      addToast(result.message, "success");
      queryClient.invalidateQueries({ queryKey: ["tweaks"] });
      queryClient.invalidateQueries({ queryKey: ["audit"] });
    },
    onError: (error: any) => {
      addToast(error.message || "Failed to revert tweak", "error");
    },
  });

  const categories = ["all", ...new Set(data?.tweaks.map((t) => t.category) || [])];

  const filtered = data?.tweaks.filter((t) => {
    const matchesFilter = filter === "all" || t.category === filter;
    const matchesSearch = search === "" ||
      t.name.toLowerCase().includes(search.toLowerCase()) ||
      t.description.toLowerCase().includes(search.toLowerCase());
    return matchesFilter && matchesSearch;
  });

  const safeCount = filtered?.filter((t) => t.risk === "Safe").length || 0;
  const moderateCount = filtered?.filter((t) => t.risk === "Moderate").length || 0;
  const advancedCount = filtered?.filter((t) => t.risk === "Advanced").length || 0;

  return (
    <div className="space-y-6 max-w-5xl">
      <div>
        <h1 className="text-2xl font-bold">Tweaks</h1>
        <p className="text-zinc-400 text-sm mt-1">
          Safe, reversible Windows optimizations
        </p>
      </div>

      <div className="flex flex-col sm:flex-row gap-3">
        <div className="relative flex-1">
          <input
            type="text"
            placeholder="Search tweaks..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="w-full bg-surface-elevated border border-surface-border rounded-lg px-4 py-2.5 text-sm text-zinc-100 placeholder-zinc-600 focus:outline-none focus:border-brand-600 focus:ring-1 focus:ring-brand-600/30 transition-all"
          />
        </div>
        <div className="flex gap-2 flex-wrap">
          {categories.map((cat) => (
            <button
              key={cat}
              onClick={() => setFilter(cat)}
              className={`px-3 py-2 text-sm rounded-lg transition-all font-medium ${
                filter === cat
                  ? "bg-brand-600 text-white shadow-lg shadow-brand-600/20"
                  : "bg-zinc-800 text-zinc-400 hover:bg-zinc-700 hover:text-zinc-200"
              }`}
            >
              {categoryLabels[cat] || cat}
            </button>
          ))}
        </div>
      </div>

      <div className="flex gap-4 text-xs text-zinc-500">
        <span className="flex items-center gap-1.5">
          <span className="w-2 h-2 rounded-full bg-emerald-500" />
          {safeCount} Safe
        </span>
        <span className="flex items-center gap-1.5">
          <span className="w-2 h-2 rounded-full bg-amber-500" />
          {moderateCount} Moderate
        </span>
        <span className="flex items-center gap-1.5">
          <span className="w-2 h-2 rounded-full bg-red-500" />
          {advancedCount} Advanced
        </span>
      </div>

      {isLoading ? (
        <div className="flex items-center justify-center py-12">
          <div className="w-8 h-8 border-2 border-brand-600 border-t-transparent rounded-full animate-spin" />
        </div>
      ) : (
        <div className="space-y-4">
          {filtered?.map((tweak) => (
            <TweakCard
              key={tweak.id}
              tweak={tweak}
              onApply={(id) => applyMutation.mutate(id)}
              onRevert={(id) => revertMutation.mutate(id)}
              isApplying={applyMutation.variables === tweak.id && applyMutation.isPending}
              isReverting={revertMutation.variables === tweak.id && revertMutation.isPending}
            />
          ))}
          {filtered?.length === 0 && (
            <div className="text-center py-12 text-zinc-500">
              No tweaks match your search.
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default TweaksPage;
