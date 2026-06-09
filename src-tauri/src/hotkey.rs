use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_global_shortcut::GlobalShortcutExt;

const SEARCH_HOTKEY: &str = "Alt+Space";
const WHEEL_HOTKEY: &str = "Ctrl+Space";

pub fn register_hotkeys(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let gs = app.global_shortcut();

    // Search bar hotkey
    let search_handle = app.clone();
    match gs.on_shortcut(SEARCH_HOTKEY, move |_app, _shortcut, _event| {
        if let Some(window) = search_handle.get_webview_window("search") {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }) {
        Ok(_) => log::info!("Registered search hotkey: {}", SEARCH_HOTKEY),
        Err(e) => log::error!("Failed to register search hotkey: {}", e),
    }

    // Wheel hotkey
    let wheel_handle = app.clone();
    match gs.on_shortcut(WHEEL_HOTKEY, move |_app, _shortcut, _event| {
        if let Some(window) = wheel_handle.get_webview_window("wheel") {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }) {
        Ok(_) => log::info!("Registered wheel hotkey: {}", WHEEL_HOTKEY),
        Err(e) => log::error!("Failed to register wheel hotkey: {}", e),
    }

    Ok(())
}
