/// Adapter functions that find a window by title and send input to its HWND.
use crate::pal::{window_pal, input_pal};
use crate::shared::AppError;

/// Finds the named window and types text into it via WM_CHAR.
pub fn focused_type_text(window_title: &str, text: &str) -> Result<(), AppError> {
    let hwnd = window_pal::find_window_by_partial_title(window_title)?;
    input_pal::type_text(hwnd, text)
}

/// Finds the named window and sends a key combination via WM_KEYDOWN/UP.
pub fn focused_send_keys(window_title: &str, keys: &str) -> Result<(), AppError> {
    let hwnd = window_pal::find_window_by_partial_title(window_title)?;
    input_pal::send_keys(hwnd, keys)
}

/// Finds the named window and clicks at absolute client coordinates.
pub fn click_at_adp(window_title: &str, x: i32, y: i32, button: &str) -> Result<(), AppError> {
    let hwnd = window_pal::find_window_by_partial_title(window_title)?;
    input_pal::click_at(hwnd, x, y, button)
}
