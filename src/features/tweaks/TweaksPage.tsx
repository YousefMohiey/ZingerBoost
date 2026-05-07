import React, { useState } from "react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { AlertTriangle, Check, ChevronDown, ChevronUp, Info, RotateCcw, Shield } from "lucide-react";
import { api, TweakMetadata } from "../../lib/api";

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
    <div className="bg-surface-elevated border border-surface-border rounded-xl overflow-hidden">
      <div className="p-5">
        <div className="flex items-start justify-between">
          <div className="flex-1">
            <div className="flex items-center gap-2 mb-1">
              <h3 className="font-semibold">{tweak.name}</h3>
              <span
                className={`text-xs px-2 py-0.5 rounded-full border ${
                  riskColors[tweak.risk] || "bg-zinc-700 text-zinc-300"
                }`}
              >
                {tweak.risk}
              </span>
              {tweak.requires_reboot && (
                <span className="text-xs px-2 py-0.5 rounded-full bg-blue-500/20 text-blue-400 border border-blue-500/30">
                  Reboot
                </span>
              )}
            </div>
            <p className="text-sm text-zinc-400">{tweak.description}</p>
            <div className="flex items-center gap-3 mt-3">
              <span className="text-xs text-zinc-500 bg-zinc-800 px-2 py-1 rounded">
                {categoryLabels[tweak.category] || tweak.category}
              </span>
              {tweak.requires_admin && (
                <span className="flex items-center gap-1 text-xs text-zinc-500">
                  <Shield className="w-3 h-3" /> Admin
                </span>
              )}
            </div>
          </div>
          <div className="flex items-center gap-2 ml-4">
            <button
              onClick={() => onApply(tweak.id)}
              disabled={isApplying}
              className="px-4 py-2 bg-brand-600 hover:bg-brand-500 disabled:opacity-50 text-white text-sm font-medium rounded-lg transition-colors"
            >
              {isApplying ? "Applying..." : "Apply"}
            </button>
            <button
              onClick={() => onRevert(tweak.id)}
              disabled={isReverting}
              className="px-4 py-2 bg-zinc-800 hover:bg-zinc-700 disabled:opacity-50 text-zinc-200 text-sm font-medium rounded-lg transition-colors"
            >
              <RotateCcw className="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>

      <button
        onClick={() => setExpanded(!expanded)}
        className="w-full flex items-center justify-center gap-1 py-2 text-xs text-zinc-500 hover:text-zinc-300 hover:bg-zinc-800/50 transition-colors border-t border-surface-border"
      >
        {expanded ? (
          <>
            <ChevronUp className="w-4 h-4" /> Less
          </>
        ) : (
          <>
            <ChevronDown className="w-4 h-4" /> More
          </>
        )}
      </button>

      {expanded && explanation && (
        <div className="px-5 pb-5 pt-2 border-t border-surface-border space-y-3">
          <div className="flex gap-3">
            <Info className="w-4 h-4 text-brand-500 mt-0.5 shrink-0" />
            <div>
              <div className="text-sm font-medium mb-1">What it does</div>
              <div className="text-sm text-zinc-400">{explanation.what_it_does}</div>
            </div>
          </div>
          <div className="flex gap-3">
            <Check className="w-4 h-4 text-emerald-500 mt-0.5 shrink-0" />
            <div>
              <div className="text-sm font-medium mb-1">Why it helps</div>
              <div className="text-sm text-zinc-400">{explanation.why_it_helps}</div>
            </div>
          </div>
          {explanation.potential_risks && (
            <div className="flex gap-3">
              <AlertTriangle className="w-4 h-4 text-amber-500 mt-0.5 shrink-0" />
              <div>
                <div className="text-sm font-medium mb-1">Potential risks</div>
                <div className="text-sm text-zinc-400">{explanation.potential_risks}</div>
              </div>
            </div>
          )}
          <div className="flex gap-3">
            <RotateCcw className="w-4 h-4 text-zinc-500 mt-0.5 shrink-0" />
            <div>
              <div className="text-sm font-medium mb-1">How to revert</div>
              <div className="text-sm text-zinc-400">{explanation.how_to_revert}</div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

const TweaksPage: React.FC = () => {
  const queryClient = useQueryClient();
  const [filter, setFilter] = useState<string>("all");

  const { data, isLoading } = useQuery({
    queryKey: ["tweaks"],
    queryFn: api.listTweaks,
  });

  const applyMutation = useMutation({
    mutationFn: api.applyTweak,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["tweaks"] }),
  });

  const revertMutation = useMutation({
    mutationFn: api.revertTweak,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["tweaks"] }),
  });

  const categories = ["all", ...new Set(data?.tweaks.map((t) => t.category) || [])];

  const filtered =
    filter === "all"
      ? data?.tweaks
      : data?.tweaks.filter((t) => t.category === filter);

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold">Tweaks</h1>
        <p className="text-zinc-400 text-sm mt-1">
          Safe, reversible Windows optimizations
        </p>
      </div>

      <div className="flex gap-2 flex-wrap">
        {categories.map((cat) => (
          <button
            key={cat}
            onClick={() => setFilter(cat)}
            className={`px-3 py-1.5 text-sm rounded-lg transition-colors ${
              filter === cat
                ? "bg-brand-600 text-white"
                : "bg-zinc-800 text-zinc-400 hover:bg-zinc-700 hover:text-zinc-200"
            }`}
          >
            {categoryLabels[cat] || cat}
          </button>
        ))}
      </div>

      {isLoading ? (
        <div className="text-zinc-500">Loading tweaks...</div>
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
        </div>
      )}
    </div>
  );
};

export default TweaksPage;
