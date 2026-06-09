import { invoke } from "@tauri-apps/api/core";
import type { AppConfig, SearchItem, WheelItem } from "./types";

// ─── Search ───────────────────────────────────────────

export async function search(query: string, limit?: number): Promise<SearchItem[]> {
  return invoke("search", { query, limit: limit ?? 20 });
}

export async function getRecent(limit?: number): Promise<SearchItem[]> {
  return invoke("get_recent", { limit: limit ?? 10 });
}

// ─── Windows ──────────────────────────────────────────

export async function showSearch(): Promise<void> {
  return invoke("show_search");
}

export async function hideSearch(): Promise<void> {
  return invoke("hide_search");
}

export async function showWheel(x: number, y: number): Promise<void> {
  return invoke("show_wheel", { x, y });
}

export async function hideWheel(): Promise<void> {
  return invoke("hide_wheel");
}

export async function showSettings(): Promise<void> {
  return invoke("show_settings");
}

// ─── Apps ─────────────────────────────────────────────

export async function getApps(): Promise<SearchItem[]> {
  return invoke("get_apps");
}

export async function launchApp(path: string): Promise<void> {
  return invoke("launch_app", { path });
}

// ─── Config ───────────────────────────────────────────

export async function getConfig(): Promise<AppConfig> {
  return invoke("get_config");
}

export async function updateConfig(config: AppConfig): Promise<void> {
  return invoke("update_config", { newConfig: config });
}

export async function updateWheelLayout(layout: WheelItem[]): Promise<void> {
  return invoke("update_wheel_layout", { layout });
}

// ─── Acrylic ──────────────────────────────────────────

export async function setAcrylicOpacity(windowLabel: string, opacity: number): Promise<void> {
  return invoke("set_acrylic_opacity", { windowLabel, opacity });
}
