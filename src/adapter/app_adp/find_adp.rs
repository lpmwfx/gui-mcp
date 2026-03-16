/// Orchestrator: locate a template in a captured window screenshot.
use crate::core::vision_core;
use crate::shared::AppError;
use crate::adapter::app_adp::{find_and_capture_adp, decode_template_adp};

/// Finds a template in a window screenshot and returns the match result.
pub fn find_element(
    window_title: &str,
    template_base64: &str,
    confidence: Option<f32>,
) -> Result<Option<(u32, u32, u32, u32, f32)>, AppError> {
    let (_hwnd, screenshot) = find_and_capture_adp(window_title)?;
    let template = decode_template_adp(template_base64)?;
    let (tw, th) = template.dimensions();
    match vision_core::find_template(&screenshot, &template, confidence) {
        Some(m) => Ok(Some((m.x, m.y, tw, th, m.confidence))),
        None => Ok(None),
    }
}
