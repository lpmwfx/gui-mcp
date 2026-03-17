/// Mouse click via SendInput -- requires foreground focus.
use crate::shared::AppError;
use crate::state::sizes::{KEY_EVENT_DELAY_MS, SENDINPUT_COORD_RANGE};

#[cfg(windows)]
use windows::{
    Win32::Foundation::{HWND, POINT},
    Win32::Graphics::Gdi::ClientToScreen,
    Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEINPUT,
        MOUSEEVENTF_MOVE, MOUSEEVENTF_ABSOLUTE,
    },
    Win32::UI::WindowsAndMessaging::{
        GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
    },
};

/// Builds a mouse INPUT struct with absolute screen coordinates and the given flags.
#[cfg(windows)]
fn mouse_input_pal(abs_x: i32, abs_y: i32, flags: windows::Win32::UI::Input::KeyboardAndMouse::MOUSE_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: abs_x,
                dy: abs_y,
                mouseData: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

/// Clicks at client-relative coordinates inside `hwnd` using SendInput.
/// Requires foreground focus (window is brought to front automatically).
pub fn click_at(hwnd: u64, client_x: i32, client_y: i32, button: &str) -> Result<(), AppError> {
    #[cfg(windows)]
    {
        use crate::pal::window_pal::focus_window_pal;
        focus_window_pal(hwnd)?;

        let hwnd_val = HWND(hwnd as usize as *mut _);
        let mut pt = POINT { x: client_x, y: client_y };
        // SAFETY: hwnd is a valid HWND obtained from window lookup; pt is stack-allocated.
        unsafe { let _ = ClientToScreen(hwnd_val, &mut pt as *mut POINT); }

        // SAFETY: GetSystemMetrics with SM_CXSCREEN/SM_CYSCREEN is always safe.
        let (cx, cy) = unsafe {
            (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN))
        };

        let abs_x = (pt.x as i64 * SENDINPUT_COORD_RANGE / cx as i64) as i32;
        let abs_y = (pt.y as i64 * SENDINPUT_COORD_RANGE / cy as i64) as i32;

        let (down_flags, up_flags) = super::vk_pal::parse_button_flags_pal(button);
        let move_abs = MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;

        let down_input = mouse_input_pal(abs_x, abs_y, move_abs | down_flags);
        // SAFETY: input struct is well-formed; SendInput injects hardware-level events.
        unsafe { SendInput(&[down_input], std::mem::size_of::<INPUT>() as i32) };
        std::thread::sleep(std::time::Duration::from_millis(KEY_EVENT_DELAY_MS));

        let up_input = mouse_input_pal(abs_x, abs_y, move_abs | up_flags);
        // SAFETY: input struct is well-formed; SendInput injects hardware-level events.
        unsafe { SendInput(&[up_input], std::mem::size_of::<INPUT>() as i32) };

        return Ok(());
    }

    #[cfg(not(windows))]
    {
        let _ = (hwnd, client_x, client_y, button);
        Err(AppError::InputError("Windows only".to_string()))
    }
}
