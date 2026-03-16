/// Thin wrappers forwarding text/key input to the PAL layer.
use crate::pal::input_pal;
use crate::shared::AppError;

/// Types text into the currently focused element.
pub fn type_text(text: &str) -> Result<(), AppError> {
    input_pal::type_text(text)
}

/// Sends a key combination.
pub fn send_keys(keys: &str) -> Result<(), AppError> {
    input_pal::send_keys(keys)
}
