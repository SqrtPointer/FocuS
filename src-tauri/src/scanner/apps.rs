use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri::Manager;
use crate::search::provider::{ItemAction, ItemType, SearchItem};

/// A wheel sector configuration item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelItem {
    pub sector: u8,         // 0-7
    pub title: String,
    pub icon: String,
    pub action: ItemAction,
}

/// Scan for installed applications on Windows
pub async fn scan_applications(app: &AppHandle) {
    let mut items = Vec::new();

    // Scan Start Menu shortcuts
    let start_menu_dirs: Vec<std::path::PathBuf> = vec![
        dirs::data_dir()
            .map(|d| d.join("Microsoft").join("Windows").join("Start Menu").join("Programs")),
        Some(std::path::PathBuf::from(
            "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs",
        )),
    ]
    .into_iter()
    .flatten()
    .collect();

    for dir in &start_menu_dirs {
        if let Ok(_entries) = std::fs::read_dir(dir) {
            scan_directory(dir, &mut items);
        }
    }

    // Update search engine with scanned apps
    let state = app.state::<crate::AppState>();
    let mut engine = state.search_engine.lock();
    for item in &items {
        engine.add(item.clone());
    }

    log::info!("Scanned {} applications", items.len());
}

fn scan_directory(dir: &std::path::Path, items: &mut Vec<SearchItem>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_directory(&path, items);
            } else if let Some(ext) = path.extension() {
                if ext == "lnk" || ext == "exe" {
                    let title = path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let app_path = path.to_string_lossy().to_string();

                    items.push(SearchItem {
                        id: format!("app:{}", title),
                        title: title.clone(),
                        subtitle: app_path.clone(),
                        icon: "📦".to_string(),
                        item_type: ItemType::Application,
                        search_text: title,
                        action: ItemAction::LaunchApp { path: app_path },
                        use_count: 0,
                    });
                }
            }
        }
    }
}

pub mod commands {
    use tauri::{command, State};
    use crate::AppState;
    use crate::search::provider::SearchItem;
    use tauri_plugin_shell::ShellExt;

    #[command]
    pub async fn get_apps(
        state: State<'_, AppState>,
    ) -> Result<Vec<SearchItem>, String> {
        let engine = state.search_engine.lock();
        Ok(engine.search("", 100))
    }

    #[command]
    pub async fn launch_app(app: tauri::AppHandle, path: String) -> Result<(), String> {
        app.shell()
            .command("cmd")
            .args(["/C", "start", "", &path])
            .output()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
