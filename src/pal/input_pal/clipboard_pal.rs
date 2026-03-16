/// Clipboard operations via Win32 messages  --  works on background windows.
use crate::shared::AppError;
use crate::state::sizes::{EM_SETSEL_ID, WM_COPY_ID, WM_CUT_ID, WM_PASTE_ID};

#[cfg(windows)]
use windows::{
    Win32::Foundation::{HWND, LPARAM, WPARAM},
    Win32::UI::WindowsAndMessaging::SendMessageW,
};

/// Selects all text in the edit control of `hwnd`.
pub fn select_all_pal(hwnd: u64) -> Result<(), AppError> {
    #[cfg(windows)]
    {
        let target = super::child_pal::find_input_child_pal(HWND(hwnd as usize as *mut _));
        // SAFETY: target is a valid HWND; EM_SETSEL(0, -1) selects all text.
        unsafe {
            SendMessageW(target, EM_SETSEL_ID, WPARAM(0), LPARAM(-1isize));
        }
        return Ok(());
    }

    #[cfg(not(windows))]
    {
        let _ = hwnd;
        Err(AppError::InputError("Windows only".to_string()))
    }
}

/// Copies the current selection to the clipboard.
pub fn copy_pal(hwnd: u64) -> Result<(), AppError> {
    #[cfg(windows)]
    {
        let target = super::child_pal::find_input_child_pal(HWND(hwnd as usize as *mut _));
        // SAFETY: target is a valid HWND; WM_COPY is always safe.
        unsafe {
            SendMessageW(target, WM_COPY_ID, WPARAM(0), LPARAM(0));
        }
        return Ok(());
    }

    #[cfg(not(windows))]
    {
        let _ = hwnd;
        Err(AppError::InputError("Windows only".to_string()))
    }
}

/// Cuts the current selection to the clipboard.
pub fn cut_pal(hwnd: u64) -> Result<(), AppError> {
    #[cfg(windows)]
    {
        let target = super::child_pal::find_input_child_pal(HWND(hwnd as usize as *mut _));
        // SAFETY: target is a valid HWND; WM_CUT is always safe.
        unsafe {
            SendMessageW(target, WM_CUT_ID, WPARAM(0), LPARAM(0));
        }
        return Ok(());
    }

    #[cfg(not(windows))]
    {
        let _ = hwnd;
        Err(AppError::InputError("Windows only".to_string()))
    }
}

/// Pastes clipboard content into the edit control.
pub fn paste_pal(hwnd: u64) -> Result<(), AppError> {
    #[cfg(windows)]
    {
        let target = super::child_pal::find_input_child_pal(HWND(hwnd as usize as *mut _));
        // SAFETY: target is a valid HWND; WM_PASTE is always safe.
        unsafe {
            SendMessageW(target, WM_PASTE_ID, WPARAM(0), LPARAM(0));
        }
        return Ok(());
    }

    #[cfg(not(windows))]
    {
        let _ = hwnd;
        Err(AppError::InputError("Windows only".to_string()))
    }
}
