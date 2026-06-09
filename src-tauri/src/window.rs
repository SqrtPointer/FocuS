use tauri::{AppHandle, Window};

pub mod commands {
    use tauri::{command, AppHandle, Manager};

    #[command]
    pub async fn show_search(app: AppHandle) -> Result<(), String> {
        let window = app
            .get_webview_window("search")
            .ok_or("Search window not found")?;
        window.center().map_err(|e| e.to_string())?;
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        Ok(())
    }

    #[command]
    pub async fn hide_search(app: AppHandle) -> Result<(), String> {
        let window = app
            .get_webview_window("search")
            .ok_or("Search window not found")?;
        window.hide().map_err(|e| e.to_string())?;
        Ok(())
    }

    #[command]
    pub async fn show_wheel(app: AppHandle, x: i32, y: i32) -> Result<(), String> {
        let window = app
            .get_webview_window("wheel")
            .ok_or("Wheel window not found")?;
        // Position window so the wheel center is at cursor
        let half_w = 200;
        let half_h = 200;
        window
            .set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(
                x - half_w,
                y - half_h,
            )))
            .map_err(|e| e.to_string())?;
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        Ok(())
    }

    #[command]
    pub async fn hide_wheel(app: AppHandle) -> Result<(), String> {
        let window = app
            .get_webview_window("wheel")
            .ok_or("Wheel window not found")?;
        window.hide().map_err(|e| e.to_string())?;
        Ok(())
    }

    #[command]
    pub async fn show_settings(app: AppHandle) -> Result<(), String> {
        let window = app
            .get_webview_window("settings")
            .ok_or("Settings window not found")?;
        window.center().map_err(|e| e.to_string())?;
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        Ok(())
    }
}
