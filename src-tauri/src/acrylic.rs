/// Applies WinUI Acrylic frosted glass effect to a Tauri window.
/// Currently uses CSS backdrop-filter as primary approach.
/// Native DWM Acrylic will be added in a future update.
#[allow(dead_code)]
pub fn apply_acrylic(_window: &tauri::Window) -> Result<(), String> {
    // Phase 2: Native Acrylic via DWM SetWindowCompositionAttribute
    // Requires: raw-window-handle + windows crate DWM features
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

        let clamped = opacity.clamp(0.3, 1.0);
        let js = format!(
            "document.documentElement.style.setProperty('--acrylic-opacity', '{}');",
            clamped
        );
        window.eval(&js).map_err(|e| e.to_string())?;

        Ok(())
    }
}
