/// Shared adapter helpers — window capture, template decode, PNG encode.
use image::RgbImage;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use crate::pal::window_pal;
use crate::shared::AppError;

/// Finds a window by partial title and captures its bitmap.
pub(crate) fn find_and_capture_adp(window_title: &str) -> Result<(u64, RgbImage), AppError> {
    let hwnd = window_pal::find_window_by_partial_title(window_title)?;
    let img = window_pal::capture_window(hwnd)?;
    Ok((hwnd, img))
}

/// Decodes a base64 PNG/JPEG template into an RgbImage.
pub(crate) fn decode_template_adp(template_base64: &str) -> Result<RgbImage, AppError> {
    let bytes = STANDARD.decode(template_base64)
        .map_err(|e| AppError::ImageError(e.to_string()))?;
    image::load_from_memory(&bytes)
        .map_err(|e| AppError::ImageError(e.to_string()))
        .map(|i| i.into_rgb8())
}

/// Captures `hwnd` after an action and returns it as a base64 PNG string.
pub(crate) fn after_screenshot_adp(hwnd: u64) -> Result<String, AppError> {
    let img = window_pal::capture_window(hwnd)?;
    let png = encode_png(&img)?;
    Ok(STANDARD.encode(&png))
}

/// Encodes an RgbImage to raw PNG bytes.
pub(crate) fn encode_png(img: &RgbImage) -> Result<Vec<u8>, AppError> {
    let mut buf = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut buf),
        image::ImageFormat::Png,
    )
    .map_err(|e| AppError::ImageError(e.to_string()))?;
    Ok(buf)
}
