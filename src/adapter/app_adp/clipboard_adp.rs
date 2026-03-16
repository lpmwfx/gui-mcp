/// Adapter functions for clipboard operations on a named window.
use crate::pal::{window_pal, input_pal};
use crate::shared::AppError;

/// Selects all text in the named window's edit control.
pub fn select_all_adp(window_title: &str) -> Result<(), AppError> {
    let hwnd = window_pal::find_window_by_partial_title(window_title)?;
    input_pal::select_all_pal(hwnd)
}

/// Copies the current selection in the named window.
pub fn copy_adp(window_title: &str) -> Result<(), AppError> {
    let hwnd = window_pal::find_window_by_partial_title(window_title)?;
    input_pal::copy_pal(hwnd)
}

/// Cuts the current selection in the named window.
pub fn cut_adp(window_title: &str) -> Result<(), AppError> {
    let hwnd = window_pal::find_window_by_partial_title(window_title)?;
    input_pal::cut_pal(hwnd)
}

/// Pastes clipboard content into the named window's edit control.
pub fn paste_adp(window_title: &str) -> Result<(), AppError> {
    let hwnd = window_pal::find_window_by_partial_title(window_title)?;
    input_pal::paste_pal(hwnd)
}
