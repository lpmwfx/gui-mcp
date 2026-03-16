/// Orchestrator: find a template in a window and click it.
use crate::core::vision_core;
use crate::pal::{window_pal, input_pal};
use crate::shared::AppError;
use crate::adapter::app_adp::{
    find_and_capture_adp, decode_template_adp, after_screenshot_adp,
};

/// Finds a template and clicks it; returns click coords, confidence, and after-screenshot.
pub fn click_element(
    window_title: &str,
    template_base64: &str,
    confidence: Option<f32>,
    button: Option<&str>,
) -> Result<(i32, i32, f32, String), AppError> {
    let (hwnd, screenshot) = find_and_capture_adp(window_title)?;
    let template = decode_template_adp(template_base64)?;
    let threshold = confidence.unwrap_or(crate::state::limits::DEFAULT_CONFIDENCE);
    let m = vision_core::find_template(&screenshot, &template, Some(threshold))
        .ok_or(AppError::TemplateNotFound { confidence: 0.0, threshold })?;

    let (rect_left, rect_top, _, _) = window_pal::get_window_rect(hwnd)?;
    let abs_x = rect_left + m.x as i32;
    let abs_y = rect_top + m.y as i32;
    input_pal::click_at(abs_x, abs_y, button.unwrap_or("left"))?;

    let after_png = after_screenshot_adp(hwnd)?;
    Ok((abs_x, abs_y, m.confidence, after_png))
}
