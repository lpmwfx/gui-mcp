/// Enumerate all visible windows and return their titles.
use crate::shared::AppError;
use crate::state::sizes::{TITLE_BUF_PADDING, WINDOW_TITLE_BUF};

#[cfg(windows)]
use windows::{
    Win32::Foundation::{BOOL, HWND, LPARAM},
    Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible,
    },
};

#[allow(non_camel_case_types)]
#[cfg(windows)]
struct CollectState_pal {
    titles: Vec<String>,
}

#[cfg(windows)]
fn read_title_pal(hwnd: HWND) -> Option<String> {
    // SAFETY: hwnd comes from EnumWindows  --  a live system-managed handle.
    let len = unsafe { GetWindowTextLengthW(hwnd) };
    if len == 0 {
        return None;
    }
    let buf_len = ((len as usize) + TITLE_BUF_PADDING).min(WINDOW_TITLE_BUF);
    let mut buf = vec![0u16; buf_len];
    // SAFETY: buf has capacity buf_len, sufficient for the GetWindowTextW result.
    let written = unsafe { GetWindowTextW(hwnd, &mut buf) };
    if written > 0 {
        Some(String::from_utf16_lossy(&buf[..written as usize]))
    } else {
        None
    }
}

#[cfg(windows)]
// SAFETY: callback is only invoked by EnumWindows with a valid lparam pointer.
unsafe extern "system" fn list_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    // SAFETY: lparam is a valid *mut CollectState_pal cast to isize, alive for EnumWindows duration.
    let collect_pal = &mut *(lparam.0 as *mut CollectState_pal);
    if !IsWindowVisible(hwnd).as_bool() {
        return BOOL(1);
    }
    if let Some(title) = read_title_pal(hwnd) {
        collect_pal.titles.push(title);
    }
    BOOL(1)
}

/// Returns titles of all visible windows with non-empty titles, sorted alphabetically.
pub fn list_window_titles_pal() -> Result<Vec<String>, AppError> {
    #[cfg(windows)]
    {
        #[allow(non_camel_case_types)]
        let mut collect_pal = CollectState_pal { titles: Vec::new() };
        // SAFETY: collect_pal lives for the entire EnumWindows call; callback casts lparam back.
        unsafe {
            let _ = EnumWindows(
                Some(list_callback),
                LPARAM(&mut collect_pal as *mut _ as isize),
            );
        }
        collect_pal.titles.sort();
        return Ok(collect_pal.titles);
    }

    #[cfg(not(windows))]
    Ok(Vec::new())
}
