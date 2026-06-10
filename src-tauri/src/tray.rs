use tauri::Manager;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

pub fn create_tray(app: &tauri::App) {
    let icon = match app.default_window_icon() {
        Some(icon) => icon.clone(),
        None => {
            log::error!("No default window icon available — tray not created");
            return;
        }
    };

    let quit = match MenuItemBuilder::with_id("quit", "Exit").build(app) {
        Ok(item) => item,
        Err(e) => {
            log::error!("Failed to create tray menu item: {}", e);
            return;
        }
    };

    let menu = match MenuBuilder::new(app).item(&quit).build() {
        Ok(m) => m,
        Err(e) => {
            log::error!("Failed to create tray menu: {}", e);
            return;
        }
    };

    match TrayIconBuilder::new()
        .icon(icon)
        .tooltip("FocuS")
        .menu(&menu)
        .on_menu_event(|app, event| {
            if event.id().as_ref() == "quit" {
                app.exit(0);
            }
        })
        .on_tray_icon_event(|tray_icon, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray_icon.app_handle();
                if let Some(window) = app.get_webview_window("search") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)
    {
        Ok(_) => log::info!("System tray created"),
        Err(e) => log::error!("Failed to create tray: {}", e),
    }
}
