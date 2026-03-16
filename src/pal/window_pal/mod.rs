/// Win32 HWND window discovery and rect query.
mod capture_pal;
/// GDI palette abstraction and color management utilities for graphics operations.
pub(crate) mod gdi_pal;

pub use capture_pal::capture_window;
pub(crate) use gdi_pal::gdi_capture_raw_pal;

use crate::shared::AppError;
use crate::state::sizes::{TITLE_BUF_PADDING, WINDOW_TITLE_BUF};

#[cfg(windows)]
use windows::{
    Win32::Foundation::{BOOL, HWND, LPARAM, RECT},
    Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetWindowTextLengthW, GetWindowTextW,
        IsWindowVisible, GetWindowRect as Win32GetWindowRect,
    },
};

#[allow(non_camel_case_types)]
#[cfg(windows)]
struct SearchState_pal {
    needle: String,
    result: Option<HWND>,
}

#[cfg(windows)]
unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    // SAFETY: lparam is a valid *mut SearchState_pal cast to isize, alive for EnumWindows duration.
    let search_pal = &mut *(lparam.0 as *mut SearchState_pal);

    if !IsWindowVisible(hwnd).as_bool() {
        return BOOL(1);
    }

    let len = GetWindowTextLengthW(hwnd);
    if len == 0 {
        return BOOL(1);
    }

    let buf_len = ((len as usize) + TITLE_BUF_PADDING).min(WINDOW_TITLE_BUF);
    let mut buf = vec![0u16; buf_len];
    let written = GetWindowTextW(hwnd, &mut buf);
    if written == 0 {
        return BOOL(1);
    }

    let title = String::from_utf16_lossy(&buf[..written as usize]);
    if title.to_lowercase().contains(search_pal.needle.as_str()) {
        search_pal.result = Some(hwnd);
        return BOOL(0);
    }

    BOOL(1)
}

/// Finds the first visible window whose title contains `partial` (case-insensitive).
pub fn find_window_by_partial_title(partial: &str) -> Result<u64, AppError> {
    #[cfg(windows)]
    {
        #[allow(non_camel_case_types)]
        let mut search_pal = SearchState_pal {
            needle: partial.to_lowercase(),
            result: None,
        };
        // SAFETY: search_pal lives for the entire EnumWindows call; callback casts lparam back.
        unsafe {
            let _ = EnumWindows(
                Some(enum_callback),
                LPARAM(&mut search_pal as *mut _ as isize),
            );
        }
        return match search_pal.result {
            Some(hwnd) => Ok(hwnd.0 as usize as u64),
            None => Err(AppError::WindowNotFound(partial.to_string())),
        };
    }

    #[cfg(not(windows))]
    {
        let _ = partial;
        Err(AppError::WindowNotFound("Windows only".to_string()))
    }
}

/// Returns the screen rect (left, top, right, bottom) for `hwnd`.
pub fn get_window_rect(hwnd: u64) -> Result<(i32, i32, i32, i32), AppError> {
    #[cfg(windows)]
    {
        let hwnd_val = HWND(hwnd as usize as *mut _);
        let mut rect = RECT::default();
        // SAFETY: hwnd is a valid HWND obtained from find_window_by_partial_title via EnumWindows.
        unsafe {
            Win32GetWindowRect(hwnd_val, &mut rect)
                .map_err(|e| AppError::CaptureFailed(e.to_string()))?;
        }
        return Ok((rect.left, rect.top, rect.right, rect.bottom));
    }

    #[cfg(not(windows))]
    {
        let _ = hwnd;
        Err(AppError::CaptureFailed("Windows only".to_string()))
    }
}
