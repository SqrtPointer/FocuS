import { writable } from "svelte/store";
import type { AppConfig, SearchItem, WheelItem } from "./types";

/** Current active view / window type */
export const currentView = writable<string>("search");

/** Current search query */
export const searchQuery = writable<string>("");

/** Search results */
export const searchResults = writable<SearchItem[]>([]);

/** Wheel items (8 sectors) */
export const wheelItems = writable<WheelItem[]>([]);

/** Active sector in wheel (-1 = dead zone, null = none selected) */
export const activeWheelSector = writable<number | null>(null);

/** App configuration */
export const appConfig = writable<AppConfig | null>(null);
