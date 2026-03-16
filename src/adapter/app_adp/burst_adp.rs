/// Captures a rapid burst of screenshots for near-live GUI viewing.
use base64::{Engine as _, engine::general_purpose::STANDARD};
use crate::shared::AppError;
use crate::adapter::app_adp::{find_and_capture_adp, encode_png};
use crate::state::sizes::SCREENSHOT_INTERVAL_MS;

/// Captures `count` screenshots with a short delay between each.
/// Returns a vec of (base64_png, width, height) tuples.
pub fn screenshot_burst(
    window_title: &str,
    count: usize,
) -> Result<Vec<(String, u32, u32)>, AppError> {
    let mut results = Vec::with_capacity(count);
    for i in 0..count {
        let (_hwnd, img) = find_and_capture_adp(window_title)?;
        let (w, h) = img.dimensions();
        let png = encode_png(&img)?;
        results.push((STANDARD.encode(&png), w, h));
        if i + 1 < count {
            std::thread::sleep(std::time::Duration::from_millis(SCREENSHOT_INTERVAL_MS));
        }
    }
    Ok(results)
}
