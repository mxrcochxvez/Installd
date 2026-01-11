use base64::{engine::general_purpose::STANDARD, Engine};
use image::{ImageBuffer, Rgba};
use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;
use std::sync::Mutex;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits, ReleaseDC, SelectObject,
    BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
};
use windows::Win32::UI::Shell::ExtractIconExW;
use windows::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, HICON, ICONINFO};

// Simple icon cache to avoid repeated extraction
lazy_static::lazy_static! {
    static ref ICON_CACHE: Mutex<HashMap<String, Option<String>>> = Mutex::new(HashMap::new());
}

/// Extract an icon from an executable or icon path and return as base64 PNG data URL
pub fn extract_icon(icon_path: &str) -> Option<String> {
    if icon_path.is_empty() {
        return None;
    }

    // Check cache first
    {
        let cache = ICON_CACHE.lock().ok()?;
        if let Some(cached) = cache.get(icon_path) {
            return cached.clone();
        }
    }

    // Parse the icon path - can be "path.exe,index" or just "path.ico"
    let (file_path, icon_index) = parse_icon_path(icon_path);

    // Check if file exists
    if !Path::new(&file_path).exists() {
        cache_result(icon_path, None);
        return None;
    }

    // Try to extract the icon
    let result = extract_icon_from_file(&file_path, icon_index);

    // Cache the result
    cache_result(icon_path, result.clone());

    result
}

fn cache_result(key: &str, value: Option<String>) {
    if let Ok(mut cache) = ICON_CACHE.lock() {
        cache.insert(key.to_string(), value);
    }
}

fn parse_icon_path(path: &str) -> (String, i32) {
    // Handle paths like "C:\Program Files\App\app.exe,0" or "path.exe,-123"
    let path = path.trim().trim_matches('"');

    if let Some(comma_pos) = path.rfind(',') {
        if let Ok(index) = path[comma_pos + 1..].trim().parse::<i32>() {
            return (path[..comma_pos].to_string(), index);
        }
    }

    (path.to_string(), 0)
}

fn extract_icon_from_file(file_path: &str, icon_index: i32) -> Option<String> {
    unsafe {
        // Convert path to wide string
        let wide_path: Vec<u16> = file_path.encode_utf16().chain(std::iter::once(0)).collect();

        let mut large_icon: HICON = HICON::default();
        let mut small_icon: HICON = HICON::default();

        // Extract the icon
        let count = ExtractIconExW(
            PCWSTR(wide_path.as_ptr()),
            icon_index,
            Some(&mut large_icon),
            Some(&mut small_icon),
            1,
        );

        if count == 0 || large_icon.is_invalid() {
            // Try with index 0 if specific index failed
            if icon_index != 0 {
                let count = ExtractIconExW(
                    PCWSTR(wide_path.as_ptr()),
                    0,
                    Some(&mut large_icon),
                    Some(&mut small_icon),
                    1,
                );
                if count == 0 || large_icon.is_invalid() {
                    return None;
                }
            } else {
                return None;
            }
        }

        // Convert icon to PNG
        let result = icon_to_png(large_icon);

        // Cleanup
        if !large_icon.is_invalid() {
            let _ = DestroyIcon(large_icon);
        }
        if !small_icon.is_invalid() {
            let _ = DestroyIcon(small_icon);
        }

        result
    }
}

unsafe fn icon_to_png(icon: HICON) -> Option<String> {
    // Get icon info
    let mut icon_info = ICONINFO::default();
    if GetIconInfo(icon, &mut icon_info).is_err() {
        return None;
    }

    // Get the color bitmap dimensions
    let hdc_screen = GetDC(HWND::default());
    let hdc_mem = CreateCompatibleDC(hdc_screen);

    // Select the color bitmap
    let old_bitmap = SelectObject(hdc_mem, icon_info.hbmColor);

    // Get bitmap info
    let mut bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: 32,
            biHeight: -32, // Top-down
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0 as u32,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [Default::default()],
    };

    // Allocate buffer for pixels (32x32 RGBA)
    let mut pixels: Vec<u8> = vec![0; 32 * 32 * 4];

    // Get the bitmap bits
    let result = GetDIBits(
        hdc_mem,
        icon_info.hbmColor,
        0,
        32,
        Some(pixels.as_mut_ptr() as *mut _),
        &mut bmi,
        DIB_RGB_COLORS,
    );

    // Cleanup GDI objects
    SelectObject(hdc_mem, old_bitmap);
    let _ = DeleteDC(hdc_mem);
    ReleaseDC(HWND::default(), hdc_screen);

    if !icon_info.hbmColor.is_invalid() {
        let _ = DeleteObject(icon_info.hbmColor);
    }
    if !icon_info.hbmMask.is_invalid() {
        let _ = DeleteObject(icon_info.hbmMask);
    }

    if result == 0 {
        return None;
    }

    // Convert BGRA to RGBA
    for chunk in pixels.chunks_exact_mut(4) {
        chunk.swap(0, 2); // Swap B and R
    }

    // Create image from pixels
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(32, 32, pixels)?;

    // Encode to PNG
    let mut png_bytes = Vec::new();
    let mut cursor = Cursor::new(&mut png_bytes);
    img.write_to(&mut cursor, image::ImageFormat::Png).ok()?;

    // Convert to base64 data URL
    let base64_data = STANDARD.encode(&png_bytes);
    Some(format!("data:image/png;base64,{}", base64_data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_icon_path() {
        assert_eq!(
            parse_icon_path("C:\\app.exe,0"),
            ("C:\\app.exe".to_string(), 0)
        );
        assert_eq!(
            parse_icon_path("C:\\app.exe,-1"),
            ("C:\\app.exe".to_string(), -1)
        );
        assert_eq!(
            parse_icon_path("\"C:\\app.exe\",0"),
            ("C:\\app.exe".to_string(), 0)
        );
        assert_eq!(
            parse_icon_path("C:\\app.ico"),
            ("C:\\app.ico".to_string(), 0)
        );
    }
}
