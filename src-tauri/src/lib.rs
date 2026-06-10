mod acrylic;
mod config;
mod cursor;
mod hotkey;
mod plugins;
mod scanner;
mod search;
mod tray;
mod window;

use std::sync::Arc;
use parking_lot::Mutex;
use tauri::Manager;

pub struct AppState {
    pub config: Arc<Mutex<config::Config>>,
    pub search_engine: Arc<Mutex<search::engine::SearchEngine>>,
}

/// Ensure only one instance of the app runs.
/// Uses a simple lock file. The file is cleaned up when the app exits normally.
fn check_single_instance() -> bool {
    let lock_path = dirs::data_dir()
        .unwrap_or_default()
        .join("FocuS")
        .join("instance.lock");

    if lock_path.exists() {
        return false;
    }

    if let Some(parent) = lock_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&lock_path, std::process::id().to_string());
    true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if !check_single_instance() {
        log::info!("Another instance is already running. Exiting.");
        return;
    }

    env_logger::init();

    let config = config::Config::load().unwrap_or_default();
    let state = AppState {
        config: Arc::new(Mutex::new(config)),
        search_engine: Arc::new(Mutex::new(search::engine::SearchEngine::new())),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .manage(state)
        .setup(|app| {
            log::info!("FocuS starting up...");

            // Initialize system tray
            tray::create_tray(app)?;

            // Register global hotkeys
            let handle = app.handle().clone();
            hotkey::register_hotkeys(handle)?;

            // Initialize app scanner (background)
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                scanner::apps::scan_applications(&handle).await;
            });

            // --- Window focus-out → hide ---
            // Search window: hide on blur
            if let Some(win) = app.get_webview_window("search") {
                let win_clone = win.clone();
                win.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        let _ = win_clone.hide();
                    }
                });
            }
            // Wheel window: hide on blur (user clicks away)
            if let Some(win) = app.get_webview_window("wheel") {
                let win_clone = win.clone();
                win.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        let _ = win_clone.hide();
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            search::commands::search,
            search::commands::get_recent,
            window::commands::show_search,
            window::commands::hide_search,
            window::commands::show_wheel,
            window::commands::hide_wheel,
            window::commands::show_settings,
            config::commands::get_config,
            config::commands::update_config,
            config::commands::update_wheel_layout,
            scanner::apps::commands::get_apps,
            scanner::apps::commands::launch_app,
            acrylic::commands::set_acrylic_opacity,
        ])
        .run(tauri::generate_context!())
        .expect("error while running FocuS");
}
