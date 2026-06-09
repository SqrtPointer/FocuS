use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchBarAppearance {
    #[serde(default = "default_search_opacity")]
    pub acrylic_opacity: f32,
    #[serde(default = "default_search_tint")]
    pub acrylic_tint: String,
}

impl Default for SearchBarAppearance {
    fn default() -> Self {
        Self {
            acrylic_opacity: default_search_opacity(),
            acrylic_tint: default_search_tint(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelAppearance {
    #[serde(default = "default_wheel_opacity")]
    pub acrylic_opacity: f32,
    #[serde(default = "default_wheel_tint")]
    pub acrylic_tint: String,
    #[serde(default = "default_border_color")]
    pub border_color: String,
    #[serde(default = "default_border_width")]
    pub border_width: u32,
    #[serde(default = "default_border_glow")]
    pub border_glow: u32,
}

impl Default for WheelAppearance {
    fn default() -> Self {
        Self {
            acrylic_opacity: default_wheel_opacity(),
            acrylic_tint: default_wheel_tint(),
            border_color: default_border_color(),
            border_width: default_border_width(),
            border_glow: default_border_glow(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appearance {
    #[serde(default)]
    pub search_bar: SearchBarAppearance,
    #[serde(default)]
    pub wheel: WheelAppearance,
    #[serde(default = "default_theme")]
    pub theme: String,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            search_bar: SearchBarAppearance::default(),
            wheel: WheelAppearance::default(),
            theme: default_theme(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotkeys {
    #[serde(default = "default_search_hotkey")]
    pub search: String,
    #[serde(default = "default_wheel_hotkey")]
    pub wheel: String,
}

impl Default for Hotkeys {
    fn default() -> Self {
        Self {
            search: default_search_hotkey(),
            wheel: default_wheel_hotkey(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConfig {
    #[serde(default = "default_index_paths")]
    pub paths: Vec<String>,
    #[serde(default = "default_exclude")]
    pub exclude: Vec<String>,
}

impl Default for IndexConfig {
    fn default() -> Self {
        Self {
            paths: default_index_paths(),
            exclude: default_exclude(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub hotkeys: Hotkeys,
    #[serde(default)]
    pub appearance: Appearance,
    #[serde(default)]
    pub index: IndexConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hotkeys: Hotkeys {
                search: default_search_hotkey(),
                wheel: default_wheel_hotkey(),
            },
            appearance: Appearance {
                search_bar: SearchBarAppearance {
                    acrylic_opacity: default_search_opacity(),
                    acrylic_tint: default_search_tint(),
                },
                wheel: WheelAppearance {
                    acrylic_opacity: default_wheel_opacity(),
                    acrylic_tint: default_wheel_tint(),
                    border_color: default_border_color(),
                    border_width: default_border_width(),
                    border_glow: default_border_glow(),
                },
                theme: default_theme(),
            },
            index: IndexConfig {
                paths: default_index_paths(),
                exclude: default_exclude(),
            },
        }
    }
}

impl Config {
    pub fn load() -> Option<Self> {
        let path = config_path()?;
        if path.exists() {
            let content = fs::read_to_string(&path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            let config = Self::default();
            config.save()?;
            Some(config)
        }
    }

    pub fn save(&self) -> Option<()> {
        let path = config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok()?;
        }
        let content = serde_json::to_string_pretty(self).ok()?;
        fs::write(&path, content).ok()?;
        Some(())
    }
}

fn config_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join("FocuS"))
}

fn config_path() -> Option<PathBuf> {
    config_dir().map(|d| d.join("config.json"))
}

pub fn wheel_layout_path() -> Option<PathBuf> {
    config_dir().map(|d| d.join("wheel.json"))
}

// Default value functions
fn default_search_opacity() -> f32 { 0.75 }
fn default_search_tint() -> String { "#1a1a1a".into() }
fn default_wheel_opacity() -> f32 { 0.7 }
fn default_wheel_tint() -> String { "#202020".into() }
fn default_border_color() -> String { "#60CDFF".into() }
fn default_border_width() -> u32 { 2 }
fn default_border_glow() -> u32 { 8 }
fn default_theme() -> String { "dark".into() }
fn default_search_hotkey() -> String { "Alt+Space".into() }
fn default_wheel_hotkey() -> String { "Ctrl+Space".into() }
fn default_index_paths() -> Vec<String> { vec!["Documents".into(), "Desktop".into()] }
fn default_exclude() -> Vec<String> { vec![".git".into(), "node_modules".into()] }

pub mod commands {
    use tauri::{command, State};
    use crate::AppState;

    #[command]
    pub async fn get_config(state: State<'_, AppState>) -> Result<crate::config::Config, String> {
        let config = state.config.lock().clone();
        Ok(config)
    }

    #[command]
    pub async fn update_config(
        state: State<'_, AppState>,
        new_config: crate::config::Config,
    ) -> Result<(), String> {
        new_config.save().ok_or("Failed to save config")?;
        *state.config.lock() = new_config;
        Ok(())
    }

    #[command]
    pub async fn update_wheel_layout(
        layout: Vec<crate::scanner::apps::WheelItem>,
    ) -> Result<(), String> {
        let path = crate::config::wheel_layout_path().ok_or("No config dir")?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let content = serde_json::to_string_pretty(&layout).map_err(|e| e.to_string())?;
        std::fs::write(&path, content).map_err(|e| e.to_string())?;
        Ok(())
    }
}
