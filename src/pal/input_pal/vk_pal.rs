/// Virtual key code mapping and key combo parsing.
use crate::shared::AppError;
use crate::state::sizes::{
    KEY_EVENT_DELAY_MS, LPARAM_PREV_STATE_BIT, LPARAM_SCANCODE_SHIFT, LPARAM_TRANSITION_BIT,
    WM_KEYDOWN_ID, WM_KEYUP_ID,
    WM_LBUTTONDOWN_ID, WM_LBUTTONUP_ID, WM_RBUTTONDOWN_ID, WM_RBUTTONUP_ID,
};

#[cfg(windows)]
use windows::{
    Win32::Foundation::{HWND, LPARAM, WPARAM},
    Win32::UI::WindowsAndMessaging::PostMessageW,
    Win32::UI::Input::KeyboardAndMouse::{MapVirtualKeyW, MAP_VIRTUAL_KEY_TYPE},
};

/// Maps virtual key names to their corresponding key codes for the PAL keyboard layout. Each tuple contains a key name string and its associated u16 key code.
pub(crate) static VK_MAP_PAL: &[(&str, u16)] = &[
    ("ctrl",      0x11),
    ("control",   0x11),
    ("alt",       0x12),
    ("shift",     0x10),
    ("win",       0x5B),
    ("meta",      0x5B),
    ("super",     0x5B),
    ("enter",     0x0D),
    ("return",    0x0D),
    ("escape",    0x1B),
    ("esc",       0x1B),
    ("tab",       0x09),
    ("backspace", 0x08),
    ("delete",    0x2E),
    ("del",       0x2E),
    ("home",      0x24),
    ("end",       0x23),
    ("pageup",    0x21),
    ("pagedown",  0x22),
    ("up",        0x26),
    ("down",      0x28),
    ("left",      0x25),
    ("right",     0x27),
    ("space",     0x20),
    ("f1",        0x70),
    ("f2",        0x71),
    ("f3",        0x72),
    ("f4",        0x73),
    ("f5",        0x74),
    ("f6",        0x75),
    ("f7",        0x76),
    ("f8",        0x77),
    ("f9",        0x78),
    ("f10",       0x79),
    ("f11",       0x7A),
    ("f12",       0x7B),
];

/// Parses a virtual key code from a key name, looking it up in VK_MAP_PAL or treating a single character as its ASCII value.
pub(crate) fn parse_vk_pal(name: &str) -> Option<u16> {
    let s = name.to_ascii_lowercase();
    if let Some((_, vk)) = VK_MAP_PAL.iter().find(|(k, _)| *k == s) {
        return Some(*vk);
    }
    let mut chars = s.chars();
    let c = chars.next()?;
    if chars.next().is_none() { Some(c.to_ascii_uppercase() as u16) } else { None }
}

/// Returns Windows message IDs for button down and up events based on the button name, defaulting to left button if the name is not recognized.
pub(crate) fn parse_button_msgs_pal(btn: &str) -> (u32, u32) {
    static BTN_MAP_PAL: &[(&str, (u32, u32))] = &[
        ("right", (WM_RBUTTONDOWN_ID, WM_RBUTTONUP_ID)),
    ];
    let s = btn.to_ascii_lowercase();
    BTN_MAP_PAL.iter()
        .find(|(k, _)| *k == s)
        .map(|(_, v)| *v)
        .unwrap_or((WM_LBUTTONDOWN_ID, WM_LBUTTONUP_ID))
}

/// Parses a keyboard combo string (e.g., "Ctrl+Shift+A") into modifier keys and a final key.
/// Splits on '+' with the last component as the final key and preceding components as modifier keys.
/// Returns a tuple of (modifier virtual keys, final virtual key) or an error if any key is invalid.
pub(crate) fn parse_combo_vk_pal(keys: &str) -> Result<(Vec<u16>, u16), AppError> {
    let parts: Vec<&str> = keys.split('+').collect();
    let (last, modifiers) = parts
        .split_last()
        .ok_or_else(|| AppError::InputError("empty key string".to_string()))?;
    let final_vk = parse_vk_pal(last)
        .ok_or_else(|| AppError::InputError(format!("unknown key: {last}")))?;
    let mod_vks: Vec<u16> = modifiers.iter().filter_map(|s| parse_vk_pal(s)).collect();
    Ok((mod_vks, final_vk))
}

#[cfg(windows)]
fn make_key_lparam_pal(vk: u16, is_up: bool) -> LPARAM {
    // SAFETY: MapVirtualKeyW with MAPVK_VK_TO_VSC is always safe.
    let scan = unsafe { MapVirtualKeyW(vk as u32, MAP_VIRTUAL_KEY_TYPE(0)) };
    let mut lp: u32 = 1;
    lp |= (scan & 0xFF) << LPARAM_SCANCODE_SHIFT;
    if is_up {
        lp |= 1 << LPARAM_PREV_STATE_BIT;
        lp |= 1 << LPARAM_TRANSITION_BIT;
    }
    LPARAM(lp as isize)
}

#[cfg(windows)]
/// Simulates pressing a keyboard combination with modifiers by posting key down and key up messages to the target window in the correct sequence.
pub(crate) fn post_combo_pal(hwnd_val: HWND, mod_vks: &[u16], final_vk: u16) {
    // SAFETY: hwnd valid; all VK codes are valid virtual key constants.
    unsafe {
        for &vk in mod_vks {
            let _ = PostMessageW(hwnd_val, WM_KEYDOWN_ID, WPARAM(vk as usize), make_key_lparam_pal(vk, false));
        }
        let _ = PostMessageW(hwnd_val, WM_KEYDOWN_ID, WPARAM(final_vk as usize), make_key_lparam_pal(final_vk, false));
        std::thread::sleep(std::time::Duration::from_millis(KEY_EVENT_DELAY_MS));
        let _ = PostMessageW(hwnd_val, WM_KEYUP_ID, WPARAM(final_vk as usize), make_key_lparam_pal(final_vk, true));
        for &vk in mod_vks.iter().rev() {
            let _ = PostMessageW(hwnd_val, WM_KEYUP_ID, WPARAM(vk as usize), make_key_lparam_pal(vk, true));
        }
    }
}
