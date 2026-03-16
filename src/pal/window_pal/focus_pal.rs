/// Bring a window to foreground by HWND.
use crate::shared::AppError;

/// Calls SetForegroundWindow + short sleep to ensure the window receives input.
pub fn focus_window_pal(hwnd: u64) -> Result<(), AppError> {
    #[cfg(windows)]
    {
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow;

        let hwnd_val = HWND(hwnd as usize as *mut _);
        // SAFETY: hwnd is a valid HWND obtained from find_window_by_partial_title.
        unsafe { let _ = SetForegroundWindow(hwnd_val); }
        std::thread::sleep(std::time::Duration::from_millis(crate::state::sizes::FOCUS_DELAY_MS));
        return Ok(());
    }

    #[cfg(not(windows))]
    {
        let _ = hwnd;
        Err(AppError::InputError("Windows only".to_string()))
    }
}
