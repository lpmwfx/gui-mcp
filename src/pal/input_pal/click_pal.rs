/// Atomic mouse click via SendInput with MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE.
/// Combines move + button into a single SendInput call to avoid the
/// SetCursorPos / SendInput race condition on Windows 10 16299+.
use crate::shared::AppError;
use crate::state::sizes::{KEY_EVENT_DELAY_MS, SENDINPUT_COORD_RANGE};

#[cfg(windows)]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEINPUT,
    MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE, MOUSE_EVENT_FLAGS,
};

#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

/// Builds a mouse INPUT struct with absolute move + the given button flags.
#[cfg(windows)]
fn mouse_input_abs(abs_x: i32, abs_y: i32, button_flags: MOUSE_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: abs_x,
                dy: abs_y,
                mouseData: 0,
                dwFlags: MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE | button_flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

/// Clicks at window-relative coordinates inside `hwnd` using an atomic
/// move+click via SendInput (MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE).
/// Coordinates match the screenshot output (0,0 = top-left of entire window including title bar).
/// Requires foreground focus (window is brought to front automatically).
pub fn click_at(hwnd: u64, win_x: i32, win_y: i32, button: &str) -> Result<(), AppError> {
    #[cfg(windows)]
    {
        use crate::pal::window_pal::focus_window_pal;
        focus_window_pal(hwnd)?;

        let (left, top, _, _) = crate::pal::window_pal::get_window_rect(hwnd)?;
        let screen_x = left + win_x;
        let screen_y = top + win_y;

        // Convert screen coords to SendInput absolute coords (0..65535).
        // SAFETY: GetSystemMetrics is always safe to call with valid SM_ constants.
        let cx_screen = unsafe { GetSystemMetrics(SM_CXSCREEN) } as i64;
        // SAFETY: GetSystemMetrics is always safe to call with valid SM_ constants.
        let cy_screen = unsafe { GetSystemMetrics(SM_CYSCREEN) } as i64;
        let abs_x = ((screen_x as i64) * SENDINPUT_COORD_RANGE / cx_screen) as i32;
        let abs_y = ((screen_y as i64) * SENDINPUT_COORD_RANGE / cy_screen) as i32;

        let (down_flags, up_flags) = super::vk_pal::parse_button_flags_pal(button);

        // Atomic move + button-down in a single SendInput call.
        let down_input = mouse_input_abs(abs_x, abs_y, down_flags);
        // SAFETY: input struct is well-formed; SendInput injects hardware-level events.
        unsafe { SendInput(&[down_input], std::mem::size_of::<INPUT>() as i32) };
        std::thread::sleep(std::time::Duration::from_millis(KEY_EVENT_DELAY_MS));

        // Atomic move + button-up (same position).
        let up_input = mouse_input_abs(abs_x, abs_y, up_flags);
        // SAFETY: input struct is well-formed; SendInput injects hardware-level events.
        unsafe { SendInput(&[up_input], std::mem::size_of::<INPUT>() as i32) };

        return Ok(());
    }

    #[cfg(not(windows))]
    {
        let _ = (hwnd, win_x, win_y, button);
        Err(AppError::InputError("Windows only".to_string()))
    }
}
