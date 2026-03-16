/// Window info and window listing adapter functions.
use crate::pal::window_pal;
use crate::shared::AppError;

/// Returns JSON with title, rect, width, and height for the named window.
pub fn get_window_info(window_title: &str) -> Result<String, AppError> {
    let hwnd = window_pal::find_window_by_partial_title(window_title)?;
    let (left, top, right, bottom) = window_pal::get_window_rect(hwnd)?;
    Ok(format!(
        r#"{{"title":{title:?},"rect":{{"left":{left},"top":{top},"right":{right},"bottom":{bottom}}},"width":{w},"height":{h}}}"#,
        title = window_title,
        w = right - left,
        h = bottom - top,
    ))
}

/// Returns a JSON array of visible window titles, sorted alphabetically.
pub fn list_windows() -> Result<String, AppError> {
    let titles = window_pal::list_window_titles_pal()?;
    Ok(serde_json::to_string(&titles).unwrap_or_else(|_| "[]".to_string()))
}
