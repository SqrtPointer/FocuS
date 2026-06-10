/// Extract icons from .exe/.lnk files on Windows and cache as PNG.
use std::path::PathBuf;

pub fn icon_cache_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_default()
        .join("FocuS")
        .join("icons")
}

pub fn extract_and_cache(file_path: &str, app_name: &str) -> Option<String> {
    let cache_dir = icon_cache_dir();
    let _ = std::fs::create_dir_all(&cache_dir);
    let icon_path = cache_dir.join(sanitize(app_name) + ".png");

    // Return cached icon as base64 data URL
    if icon_path.exists() {
        return png_to_data_url(&icon_path);
    }

    #[cfg(windows)]
    {
        extract_windows(file_path, &icon_path)?;
        png_to_data_url(&icon_path)
    }

    #[cfg(not(windows))]
    None
}

/// Read a PNG file and return as `data:image/png;base64,...`
fn png_to_data_url(path: &PathBuf) -> Option<String> {
    use std::io::Read;
    let mut f = std::fs::File::open(path).ok()?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).ok()?;
    let b64 = base64_encode(&buf);
    Some(format!("data:image/png;base64,{}", b64))
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        out.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            out.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

#[cfg(windows)]
fn extract_windows(file_path: &str, output: &PathBuf) -> Option<()> {
    use windows::Win32::UI::Shell::{
        SHGetFileInfoW, SHGFI_FLAGS, SHGFI_ICON, SHGFI_LARGEICON, SHFILEINFOW,
    };
    use windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES;
    use windows::Win32::Graphics::Gdi::{
        CreateCompatibleDC, DeleteDC, CreateCompatibleBitmap,
        SelectObject, DeleteObject, GetDIBits,
        BITMAPINFO, BITMAPINFOHEADER,
        DIB_RGB_COLORS, BI_RGB,
        HDC, HBITMAP, HGDIOBJ, HBRUSH, DIB_USAGE,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        DrawIconEx, DestroyIcon, DI_NORMAL,
    };
    use windows::core::PCWSTR;
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let wide: Vec<u16> = OsStr::new(file_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let mut info = SHFILEINFOW::default();
    let flags = SHGFI_FLAGS(SHGFI_ICON.0 | SHGFI_LARGEICON.0);

    let result = unsafe {
        SHGetFileInfoW(
            PCWSTR::from_raw(wide.as_ptr()),
            FILE_FLAGS_AND_ATTRIBUTES::default(),
            Some(&mut info),
            std::mem::size_of::<SHFILEINFOW>() as u32,
            flags,
        )
    };

    if result == 0 || info.hIcon.is_invalid() {
        return None;
    }

    let icon = info.hIcon;
    let size = 32i32;
    let mut pixels: Vec<u8> = vec![0u8; (size * size * 4) as usize];
    let mut success = false;

    unsafe {
        let hdc: HDC = CreateCompatibleDC(None);
        let bitmap: HBITMAP = CreateCompatibleBitmap(hdc, size, size);

        if hdc.is_invalid() || bitmap.is_invalid() {
            if !bitmap.is_invalid() { let _ = DeleteObject(bitmap); }
            if !hdc.is_invalid() { let _ = DeleteDC(hdc); }
            let _ = DestroyIcon(icon);
            return None;
        }

        let old_bmp: HGDIOBJ = SelectObject(hdc, bitmap);
        let _ = DrawIconEx(hdc, 0, 0, icon, size, size, 0, HBRUSH::default(), DI_NORMAL);

        let mut bmi = BITMAPINFO::default();
        bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bmi.bmiHeader.biWidth = size;
        bmi.bmiHeader.biHeight = -(size);
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 32;
        bmi.bmiHeader.biCompression = BI_RGB.0 as u32;

        let rows = GetDIBits(
            hdc,
            bitmap,
            0,
            size as u32,
            Some(pixels.as_mut_ptr() as *mut std::ffi::c_void),
            &mut bmi as *mut _,
            DIB_RGB_COLORS,
        );

        SelectObject(hdc, old_bmp);
        let _ = DeleteObject(bitmap);
        let _ = DeleteDC(hdc);
        let _ = DestroyIcon(icon);

        success = rows != 0;
    }

    if !success {
        return None;
    }

    // BGRA → RGBA
    for chunk in pixels.chunks_exact_mut(4) {
        chunk.swap(0, 2);
    }

    let img = image::RgbaImage::from_raw(size as u32, size as u32, pixels)?;
    let mut w = std::io::Cursor::new(Vec::new());
    img.write_to(&mut w, image::ImageFormat::Png).ok()?;
    let _ = std::fs::write(output, w.into_inner());

    Some(())
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| if r#"<>:"/\|?*"#.contains(c) { '_' } else { c })
        .collect()
}
