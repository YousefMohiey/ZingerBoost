import { invoke } from "@tauri-apps/api/core";

export interface TweakMetadata {
  id: string;
  name: string;
  description: string;
  category: string;
  risk: "Safe" | "Moderate" | "Advanced";
  requires_reboot: boolean;
  requires_admin: boolean;
  affected_keys: { root: string; path: string }[];
  source_url: string | null;
}

export interface TweakResult {
  reboot_required: boolean;
  message: string;
}

export interface SystemMetrics {
  cpu_percent: number;
  ram_percent: number;
  ram_used_mb: number;
  ram_total_mb: number;
  disk_active_percent: number;
  network_down_mbps: number;
  network_up_mbps: number;
}

export interface TweakExplanation {
  what_it_does: string;
  why_it_helps: string;
  potential_risks: string | null;
  how_to_revert: string;
}

export const api = {
  listTweaks: (): Promise<{ tweaks: TweakMetadata[] }> =>
    invoke("list_tweaks"),

  applyTweak: (id: string): Promise<TweakResult> =>
    invoke("apply_tweak", { request: { id } }),

  batchApplyTweaks: (ids: string[]): Promise<[string, TweakResult][]> =>
    invoke("batch_apply_tweaks", { request: { ids } }),

  revertTweak: (id: string): Promise<TweakResult> =>
    invoke("revert_tweak", { request: { id } }),

  getMetrics: (): Promise<SystemMetrics> =>
    invoke("get_metrics"),

  getTweakExplanation: (id: string): Promise<TweakExplanation> =>
    invoke("get_tweak_explanation", { id }),
};
