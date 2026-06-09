/** Types of searchable items */
export type ItemType = "Application" | "File" | "Folder" | "SystemCommand" | "PluginCommand";

/** Action to perform when an item is selected */
export type ItemAction =
  | { type: "LaunchApp"; path: string }
  | { type: "OpenFile"; path: string }
  | { type: "OpenFolder"; path: string }
  | { type: "RunSystemCommand"; command: string }
  | { type: "RunPlugin"; plugin_id: string; action_id: string };

/** A single searchable item */
export interface SearchItem {
  id: string;
  title: string;
  subtitle: string;
  icon: string;
  item_type: ItemType;
  search_text: string;
  action: ItemAction;
  use_count: number;
}

/** Wheel sector configuration */
export interface WheelItem {
  sector: number; // 0-7
  title: string;
  icon: string;
  action: ItemAction;
}

/** Appearance configuration for search bar */
export interface SearchBarAppearance {
  acrylic_opacity: number;
  acrylic_tint: string;
}

/** Appearance configuration for wheel */
export interface WheelAppearance {
  acrylic_opacity: number;
  acrylic_tint: string;
  border_color: string;
  border_width: number;
  border_glow: number;
}

/** Full appearance config */
export interface AppearanceConfig {
  search_bar: SearchBarAppearance;
  wheel: WheelAppearance;
  theme: string;
}

/** App configuration */
export interface AppConfig {
  hotkeys: {
    search: string;
    wheel: string;
  };
  appearance: AppearanceConfig;
  index: {
    paths: string[];
    exclude: string[];
  };
}
