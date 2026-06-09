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

pub struct AppState {
    pub config: Arc<Mutex<config::Config>>,
    pub search_engine: Arc<Mutex<search::engine::SearchEngine>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
