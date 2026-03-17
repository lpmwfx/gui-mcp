/// Mouse and keyboard input via SendInput (hardware-level injection).
mod child_pal;
mod click_pal;
mod clipboard_pal;
/// Virtual-key code mapping and combo parsing.
pub(crate) mod vk_pal;

use crate::shared::AppError;
use crate::state::sizes::KEY_EVENT_DELAY_MS;

#[cfg(windows)]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT,
    KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, KEYBD_EVENT_FLAGS, VIRTUAL_KEY,
};

pub use click_pal::click_at;
pub use clipboard_pal::{select_all_pal, copy_pal, cut_pal, paste_pal};

/// Builds a single KEYBDINPUT INPUT struct.
#[cfg(windows)]
fn make_vk_input(vk: u16, flags: KEYBD_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(vk),
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

/// Builds a KEYEVENTF_UNICODE INPUT struct for a single UTF-16 code unit.
#[cfg(windows)]
fn make_unicode_input(ch: u16, flags: KEYBD_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
                wScan: ch,
                dwFlags: KEYEVENTF_UNICODE | flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

/// Types text into `hwnd` using SendInput with KEYEVENTF_UNICODE.
/// Converts `\n` to `\r` for RichEdit compatibility.
pub fn type_text(hwnd: u64, text: &str) -> Result<(), AppError> {
    #[cfg(windows)]
    {
        use crate::pal::window_pal::focus_window_pal;
        focus_window_pal(hwnd)?;

        let normalized = text.replace('\n', "\r");
        let mut inputs: Vec<INPUT> = Vec::new();

        for ch in normalized.encode_utf16() {
            inputs.push(make_unicode_input(ch, KEYBD_EVENT_FLAGS(0)));
            inputs.push(make_unicode_input(ch, KEYEVENTF_KEYUP));
        }

        if !inputs.is_empty() {
            // SAFETY: inputs are well-formed KEYBDINPUT structs for SendInput.
            unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
        }

        return Ok(());
    }

    #[cfg(not(windows))]
    {
        let _ = (hwnd, text);
        Err(AppError::InputError("Windows only".to_string()))
    }
}

/// Sends a key combination to `hwnd` using SendInput.
pub fn send_keys(hwnd: u64, keys: &str) -> Result<(), AppError> {
    #[cfg(windows)]
    {
        use crate::pal::window_pal::focus_window_pal;
        focus_window_pal(hwnd)?;

        let (mod_vks, final_vk) = vk_pal::parse_combo_vk_pal(keys)?;

        let mut down_inputs: Vec<INPUT> = Vec::new();
        for &vk in &mod_vks {
            down_inputs.push(make_vk_input(vk, KEYBD_EVENT_FLAGS(0)));
        }
        down_inputs.push(make_vk_input(final_vk, KEYBD_EVENT_FLAGS(0)));

        // SAFETY: inputs are well-formed KEYBDINPUT structs for SendInput.
        unsafe { SendInput(&down_inputs, std::mem::size_of::<INPUT>() as i32) };
        std::thread::sleep(std::time::Duration::from_millis(KEY_EVENT_DELAY_MS));

        let mut up_inputs: Vec<INPUT> = Vec::new();
        up_inputs.push(make_vk_input(final_vk, KEYEVENTF_KEYUP));
        for &vk in mod_vks.iter().rev() {
            up_inputs.push(make_vk_input(vk, KEYEVENTF_KEYUP));
        }

        // SAFETY: inputs are well-formed KEYBDINPUT structs for SendInput.
        unsafe { SendInput(&up_inputs, std::mem::size_of::<INPUT>() as i32) };

        return Ok(());
    }

    #[cfg(not(windows))]
    {
        let _ = (hwnd, keys);
        Err(AppError::InputError("Windows only".to_string()))
    }
}
