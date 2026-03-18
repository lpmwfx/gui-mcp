/// Mouse click via SetCursorPos + SendInput -- requires foreground focus.
use crate::shared::AppError;
use crate::state::sizes::KEY_EVENT_DELAY_MS;

#[cfg(windows)]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEINPUT,
    MOUSE_EVENT_FLAGS,
};

#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;

/// Builds a mouse INPUT struct with button-only flags (no move/absolute).
#[cfg(windows)]
fn button_input_pal(flags: MOUSE_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

/// Clicks at window-relative coordinates inside `hwnd` using SetCursorPos + SendInput.
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

        // SAFETY: SetCursorPos moves the cursor in physical screen coordinates (DPI-aware process).
        unsafe { SetCursorPos(screen_x, screen_y) }
            .map_err(|e| AppError::InputError(format!("SetCursorPos failed: {e}")))?;

        let (down_flags, up_flags) = super::vk_pal::parse_button_flags_pal(button);

        let down_input = button_input_pal(down_flags);
        // SAFETY: input struct is well-formed; SendInput injects hardware-level events.
        unsafe { SendInput(&[down_input], std::mem::size_of::<INPUT>() as i32) };
        std::thread::sleep(std::time::Duration::from_millis(KEY_EVENT_DELAY_MS));

        let up_input = button_input_pal(up_flags);
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
