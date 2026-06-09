use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

const SEARCH_HOTKEY: &str = "Alt+Space";
const WHEEL_HOTKEY: &str = "Ctrl+Space";

pub fn register_hotkeys(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let gs = app.global_shortcut();

    // Search bar hotkey
    let search_handle = app.clone();
    gs.on_shortcut(
        SEARCH_HOTKEY.parse::<Shortcut>()?,
        move |_app, _shortcut, _event| {
            if let Some(window) = search_handle.get_webview_window("search") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        },
    )?;

    // Wheel hotkey
    let wheel_handle = app.clone();
    gs.on_shortcut(
        WHEEL_HOTKEY.parse::<Shortcut>()?,
        move |_app, _shortcut, _event| {
            if let Some(window) = wheel_handle.get_webview_window("wheel") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        },
    )?;

    log::info!(
        "Global hotkeys registered: {}, {}",
        SEARCH_HOTKEY,
        WHEEL_HOTKEY
    );
    Ok(())
}
