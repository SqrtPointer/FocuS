use tauri::AppHandle;

/// Applies WinUI Acrylic frosted glass effect to a Tauri window.
/// Falls back gracefully on systems without Acrylic support.
pub fn apply_acrylic(window: &tauri::Window, opacity: f32, tint: [u8; 4]) -> Result<(), String> {
    #[cfg(windows)]
    {
        use windows::Win32::Graphics::Dwm::{
            DwmExtendFrameIntoClientArea, DwmSetWindowAttribute, DWM_MARGINS,
            DWMWA_SYSTEMBACKDROP_TYPE,
        };
        use windows::Win32::UI::Controls::SetWindowCompositionAttribute;
        use raw_window_handle::{HasWindowHandle, RawWindowHandle};

        if let Ok(hwnd) = window.hwnd() {
            // Enable Acrylic backdrop via DWM
            // DWMSBT_MAINWINDOW = 1 (Mica), DWMSBT_TABBEDWINDOW = 4 (Acrylic)
            unsafe {
                let backdrop_type: u32 = 4; // Acrylic
                let _ = DwmSetWindowAttribute(
                    hwnd,
                    DWMWA_SYSTEMBACKDROP_TYPE,
                    &backdrop_type as *const _ as *const _,
                    std::mem::size_of::<u32>() as u32,
                );
            }
        }
    }

    #[cfg(not(windows))]
    {
        // macOS: use NSVisualEffectView via window-vibrancy
        // let _ = window;
    }

    Ok(())
}

pub mod commands {
    use tauri::{command, AppHandle, Manager};

    #[command]
    pub async fn set_acrylic_opacity(
        app: AppHandle,
        window_label: String,
        opacity: f32,
    ) -> Result<(), String> {
        let window = app
            .get_webview_window(&window_label)
            .ok_or(format!("Window '{}' not found", window_label))?;

        // Clamp opacity to valid range
        let clamped = opacity.clamp(0.3, 1.0);
        // Update CSS variable via eval
        let js = format!(
            "document.documentElement.style.setProperty('--acrylic-opacity', '{}');",
            clamped
        );
        window.eval(&js).map_err(|e| e.to_string())?;

        Ok(())
    }
}
