use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_global_shortcut::GlobalShortcutExt;

const SEARCH_HOTKEY: &str = "Alt+Space";
// Mouse forward (XButton1) is not supported by the global-shortcut library
// (keyboard_types::Code has no mouse button variants).
// Using Alt+` (Grave) as fallback — easy to reach, rarely conflicts.
// Will be configurable via GUI settings in a future update.
const WHEEL_HOTKEY: &str = "Alt+Backquote";

pub fn register_hotkeys(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let gs = app.global_shortcut();

    // Search bar hotkey (Alt+Space)
    let search_handle = app.clone();
    match gs.on_shortcut(SEARCH_HOTKEY, move |_app, _shortcut, _event| {
        if let Some(window) = search_handle.get_webview_window("search") {
            let _ = window.center();
            let _ = window.show();
            let _ = window.set_focus();
        }
    }) {
        Ok(_) => log::info!("Registered search hotkey: {}", SEARCH_HOTKEY),
        Err(e) => log::error!("Failed to register search hotkey: {}", e),
    }

    // Wheel hotkey (Alt+` — mouse forward not supported by library)
    let wheel_handle = app.clone();
    match gs.on_shortcut(WHEEL_HOTKEY, move |_app, _shortcut, _event| {
        if let Some(window) = wheel_handle.get_webview_window("wheel") {
            let (x, y) = crate::cursor::get_cursor_pos();
            let half_w = 200i32;
            let half_h = 200i32;
            let _ = window.set_position(tauri::Position::Physical(
                tauri::PhysicalPosition::new(x - half_w, y - half_h),
            ));
            let _ = window.show();
            let _ = window.set_focus();
        }
    }) {
        Ok(_) => log::info!("Registered wheel hotkey: {}", WHEEL_HOTKEY),
        Err(e) => log::error!("Failed to register wheel hotkey: {}", e),
    }

    Ok(())
}
