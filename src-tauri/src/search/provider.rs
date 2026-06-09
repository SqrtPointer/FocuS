use serde::{Deserialize, Serialize};

/// Types of searchable items
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemType {
    Application,
    File,
    Folder,
    SystemCommand,
    PluginCommand,
}

/// A single searchable item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchItem {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub icon: String,
    pub item_type: ItemType,
    pub search_text: String,
    pub action: ItemAction,
    #[serde(default)]
    pub use_count: u32,
}

/// Action to perform when the item is selected
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ItemAction {
    LaunchApp { path: String },
    OpenFile { path: String },
    OpenFolder { path: String },
    RunSystemCommand { command: String },
    RunPlugin { plugin_id: String, action_id: String },
}
