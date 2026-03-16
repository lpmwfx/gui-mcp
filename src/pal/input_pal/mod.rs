/// Mouse and keyboard input via Win32 SendMessage/PostMessage.
/// Works on background windows without requiring foreground focus.
mod child_pal;
mod click_pal;
/// Vulkan platform abstraction layer providing low-level graphics API bindings and utilities.
pub(crate) mod vk_pal;

use crate::shared::AppError;
use crate::state::sizes::WM_CHAR_ID;

#[cfg(windows)]
use windows::{
    Win32::Foundation::{HWND, LPARAM, WPARAM},
    Win32::UI::WindowsAndMessaging::SendMessageW,
};

pub use click_pal::click_at;

/// Types text into `hwnd` via WM_CHAR messages. No foreground required.
pub fn type_text(hwnd: u64, text: &str) -> Result<(), AppError> {
    #[cfg(windows)]
    {
        let target = child_pal::find_input_child_pal(HWND(hwnd as usize as *mut _));
        for ch in text.encode_utf16() {
            // SAFETY: target is a valid HWND (either the original or a child edit control).
            unsafe {
                SendMessageW(target, WM_CHAR_ID, WPARAM(ch as usize), LPARAM(0));
            }
        }
        return Ok(());
    }

    #[cfg(not(windows))]
    {
        let _ = (hwnd, text);
        Err(AppError::InputError("Windows only".to_string()))
    }
}

/// Sends a key combination to `hwnd` via WM_KEYDOWN/WM_KEYUP. No foreground required.
pub fn send_keys(hwnd: u64, keys: &str) -> Result<(), AppError> {
    #[cfg(windows)]
    {
        let hwnd_val = child_pal::find_input_child_pal(HWND(hwnd as usize as *mut _));
        let (mod_vks, final_vk) = vk_pal::parse_combo_vk_pal(keys)?;
        vk_pal::post_combo_pal(hwnd_val, &mod_vks, final_vk);
        return Ok(());
    }

    #[cfg(not(windows))]
    {
        let _ = (hwnd, keys);
        Err(AppError::InputError("Windows only".to_string()))
    }
}
