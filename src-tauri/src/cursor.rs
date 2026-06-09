/// Returns the current global cursor position (x, y) in screen coordinates.
#[allow(dead_code)]
pub fn get_cursor_pos() -> (i32, i32) {
    #[cfg(windows)]
    {
        use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
        use windows::Win32::Foundation::POINT;
        let mut point = POINT { x: 0, y: 0 };
        unsafe {
            let _ = GetCursorPos(&mut point);
        }
        (point.x, point.y)
    }

    #[cfg(not(windows))]
    {
        (0, 0)
    }
}
