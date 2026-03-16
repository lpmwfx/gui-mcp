/// Adapter: capture a window, crop a region, return base64 PNG.
use base64::{Engine as _, engine::general_purpose::STANDARD};
use crate::core::crop_core;
use crate::shared::AppError;
use crate::adapter::app_adp::{find_and_capture_adp, encode_png};

/// Captures `window_title`, crops `(x, y, w, h)`, returns the region as base64 PNG.
pub fn crop_region_adp(
    window_title: &str,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) -> Result<String, AppError> {
    let (_hwnd, screenshot) = find_and_capture_adp(window_title)?;
    let cropped = crop_core::crop_region_core(&screenshot, x, y, w, h)
        .ok_or_else(|| AppError::ImageError("crop region out of bounds".to_string()))?;
    let png_bytes = encode_png(&cropped)?;
    Ok(STANDARD.encode(&png_bytes))
}
