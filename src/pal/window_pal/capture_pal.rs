/// Orchestrates window capture: GDI pixels -> BGRA-to-RGB -> RgbImage.
use image::RgbImage;
use crate::shared::AppError;
use crate::state::sizes::{BGRA_B_IDX, BGRA_G_IDX, BGRA_R_IDX, CAPTURE_BYTES_PER_PIXEL, RGB_CHANNELS};
use crate::pal::window_pal::gdi_capture_raw_pal;

fn bgra_to_rgb_pal(pixels: &[u8], w: i32, h: i32) -> Vec<u8> {
    let mut rgb = Vec::with_capacity((w * h) as usize * RGB_CHANNELS as usize);
    for chunk in pixels.chunks_exact(CAPTURE_BYTES_PER_PIXEL) {
        rgb.push(chunk[BGRA_R_IDX]);
        rgb.push(chunk[BGRA_G_IDX]);
        rgb.push(chunk[BGRA_B_IDX]);
    }
    rgb
}

/// Captures the window bitmap for `hwnd` and returns it as an RgbImage.
pub fn capture_window(hwnd: u64) -> Result<RgbImage, AppError> {
    let (pixels, w, h) = gdi_capture_raw_pal(hwnd)?;
    let rgb = bgra_to_rgb_pal(&pixels, w, h);
    RgbImage::from_raw(w as u32, h as u32, rgb)
        .ok_or_else(|| AppError::CaptureFailed("RgbImage::from_raw failed".to_string()))
}
