/// Mouse and keyboard input via enigo.
use crate::shared::AppError;

/// Moves the mouse to absolute screen coordinates and clicks.
pub fn click_at(_x: i32, _y: i32, _button: &str) -> Result<(), AppError> {
    todo!("implement enigo mouse click")
}

/// Types `text` into the currently focused element.
pub fn type_text(_text: &str) -> Result<(), AppError> {
    todo!("implement enigo text input")
}

/// Sends a key combination such as `ctrl+s` or `enter`.
pub fn send_keys(_keys: &str) -> Result<(), AppError> {
    todo!("implement enigo key send")
}
