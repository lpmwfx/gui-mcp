/// Captures a window screenshot and returns it as base64 PNG.
use base64::{Engine as _, engine::general_purpose::STANDARD};
use crate::shared::AppError;
use crate::adapter::app_adp::{find_and_capture_adp, encode_png};

/// Captures a window by partial title and returns the PNG as base64.
pub fn screenshot_window(window_title: &str) -> Result<(String, u32, u32, String), AppError> {
    let (_hwnd, img) = find_and_capture_adp(window_title)?;
    let (w, h) = img.dimensions();
    let png = encode_png(&img)?;
    Ok((STANDARD.encode(&png), w, h, window_title.to_string()))
}
