mod acrylic;
mod config;
mod cursor;
mod hotkey;
mod icons;
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
/// Uses a lock file with PID; cleans up stale locks from crashed instances.
fn check_single_instance() -> bool {
    let lock_path = dirs::data_dir()
        .unwrap_or_default()
        .join("FocuS")
        .join("instance.lock");

    if lock_path.exists() {
        // Check if the PID in the lock file is still alive
        if let Ok(content) = std::fs::read_to_string(&lock_path) {
            if let Ok(pid) = content.trim().parse::<u32>() {
                #[cfg(windows)]
                {
                    // Try to open the process — if we can't, it's stale
                    use windows::Win32::System::Threading::{
                        OpenProcess, GetExitCodeProcess,
                        PROCESS_QUERY_LIMITED_INFORMATION,
                    };
                    use windows::Win32::Foundation::CloseHandle;
                    unsafe {
                        let h = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid);
                        if let Ok(handle) = h {
                            let mut exit_code = 0u32;
                            let _ = GetExitCodeProcess(handle, &mut exit_code);
                            let _ = CloseHandle(handle);
                            // STILL_ACTIVE = 259
                            if exit_code == 259 {
                                log::info!("Another instance is running (PID {}), exiting.", pid);
                                return false;
                            }
                        }
                    }
                }
                // PID not alive — stale lock, remove it
                let _ = std::fs::remove_file(&lock_path);
            }
        } else {
            let _ = std::fs::remove_file(&lock_path);
        }
    }

    if let Some(parent) = lock_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&lock_path, std::process::id().to_string());
    true
}

fn cleanup_lock_file() {
    let lock_path = dirs::data_dir()
        .unwrap_or_default()
        .join("FocuS")
        .join("instance.lock");
    let _ = std::fs::remove_file(&lock_path);
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
            tray::create_tray(app);

            // Register global hotkeys
            let handle = app.handle().clone();
            if let Err(e) = hotkey::register_hotkeys(handle) {
                log::error!("Hotkey registration failed: {}", e);
            }

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
            scanner::files::commands::search_files,
            acrylic::commands::set_acrylic_opacity,
        ])
        .build(tauri::generate_context!())
        .expect("error while building FocuS")
        .run(|_app_handle, event| {
            if let tauri::RunEvent::Exit = event {
                cleanup_lock_file();
            }
        });
}
