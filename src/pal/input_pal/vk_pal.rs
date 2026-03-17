/// Virtual key code mapping and key combo parsing.
use crate::shared::AppError;

#[cfg(windows)]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
    MOUSE_EVENT_FLAGS,
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

/// Returns MOUSE_EVENT_FLAGS for button down and up based on the button name.
#[cfg(windows)]
pub(crate) fn parse_button_flags_pal(btn: &str) -> (MOUSE_EVENT_FLAGS, MOUSE_EVENT_FLAGS) {
    if btn.eq_ignore_ascii_case("right") {
        (MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP)
    } else {
        (MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP)
    }
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
