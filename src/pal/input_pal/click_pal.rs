/// Mouse click via PostMessage — works on background windows.
use crate::shared::AppError;
use crate::state::sizes::{KEY_EVENT_DELAY_MS, MOUSE_COORD_MASK, MOUSE_LPARAM_Y_SHIFT};

#[cfg(windows)]
use windows::{
    Win32::Foundation::{HWND, LPARAM, WPARAM},
    Win32::UI::WindowsAndMessaging::PostMessageW,
};

#[cfg(windows)]
fn make_mouse_lparam_pal(client_x: i32, client_y: i32) -> LPARAM {
    let packed = (client_x & MOUSE_COORD_MASK) | ((client_y & MOUSE_COORD_MASK) << MOUSE_LPARAM_Y_SHIFT);
    LPARAM(packed as isize)
}

/// Clicks at client-relative coordinates inside `hwnd`. No foreground required.
pub fn click_at(hwnd: u64, client_x: i32, client_y: i32, button: &str) -> Result<(), AppError> {
    #[cfg(windows)]
    {
        let hwnd_val = HWND(hwnd as usize as *mut _);
        let lparam = make_mouse_lparam_pal(client_x, client_y);
        let (down_msg, up_msg) = super::vk_pal::parse_button_msgs_pal(button);
        // SAFETY: hwnd is a valid HWND; messages are well-formed.
        unsafe {
            let _ = PostMessageW(hwnd_val, down_msg, WPARAM(1), lparam);
            std::thread::sleep(std::time::Duration::from_millis(KEY_EVENT_DELAY_MS));
            let _ = PostMessageW(hwnd_val, up_msg, WPARAM(0), lparam);
        }
        return Ok(());
    }

    #[cfg(not(windows))]
    {
        let _ = (hwnd, client_x, client_y, button);
        Err(AppError::InputError("Windows only".to_string()))
    }
}
