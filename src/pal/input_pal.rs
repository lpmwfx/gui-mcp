/// Mouse and keyboard input via enigo.
use enigo::{Button, Coordinate, Direction, Enigo, Key, Keyboard, Mouse, Settings};
use crate::shared::AppError;

static BTN_MAP_PAL: &[(&str, Button)] = &[
    ("right",  Button::Right),
    ("middle", Button::Middle),
];

static KEY_MAP_PAL: &[(&str, Key)] = &[
    ("ctrl",      Key::Control),
    ("control",   Key::Control),
    ("alt",       Key::Alt),
    ("shift",     Key::Shift),
    ("win",       Key::Meta),
    ("meta",      Key::Meta),
    ("super",     Key::Meta),
    ("enter",     Key::Return),
    ("return",    Key::Return),
    ("escape",    Key::Escape),
    ("esc",       Key::Escape),
    ("tab",       Key::Tab),
    ("backspace", Key::Backspace),
    ("delete",    Key::Delete),
    ("del",       Key::Delete),
    ("home",      Key::Home),
    ("end",       Key::End),
    ("pageup",    Key::PageUp),
    ("pagedown",  Key::PageDown),
    ("up",        Key::UpArrow),
    ("down",      Key::DownArrow),
    ("left",      Key::LeftArrow),
    ("right",     Key::RightArrow),
    ("f1",        Key::F1),
    ("f2",        Key::F2),
    ("f3",        Key::F3),
    ("f4",        Key::F4),
    ("f5",        Key::F5),
    ("f6",        Key::F6),
    ("f7",        Key::F7),
    ("f8",        Key::F8),
    ("f9",        Key::F9),
    ("f10",       Key::F10),
    ("f11",       Key::F11),
    ("f12",       Key::F12),
];

fn parse_button_pal(btn: &str) -> Button {
    let s = btn.to_ascii_lowercase();
    BTN_MAP_PAL
        .iter()
        .find(|(k, _)| *k == s)
        .map(|(_, v)| *v)
        .unwrap_or(Button::Left)
}

fn parse_key_pal(name: &str) -> Option<Key> {
    let s = name.to_ascii_lowercase();
    if let Some((_, v)) = KEY_MAP_PAL.iter().find(|(k, _)| *k == s) {
        return Some(*v);
    }
    let mut chars = s.chars();
    let c = chars.next()?;
    if chars.next().is_none() { Some(Key::Unicode(c)) } else { None }
}

fn collect_modifiers_pal(parts: &[&str]) -> Vec<Key> {
    parts.iter().filter_map(|s| parse_key_pal(s)).collect()
}

fn parse_combo_pal(keys: &str) -> Result<(Vec<Key>, Key), AppError> {
    let parts: Vec<&str> = keys.split('+').collect();
    let (last, rest) = parts
        .split_last()
        .ok_or_else(|| AppError::InputError("empty key string".to_string()))?;
    let final_key = parse_key_pal(last)
        .ok_or_else(|| AppError::InputError(format!("unknown key: {last}")))?;
    Ok((collect_modifiers_pal(rest), final_key))
}

fn send_combo_pal(enigo: &mut Enigo, modifiers: &[Key], final_key: Key) -> Result<(), AppError> {
    for &m in modifiers {
        enigo
            .key(m, Direction::Press)
            .map_err(|e| AppError::InputError(e.to_string()))?;
    }
    enigo
        .key(final_key, Direction::Click)
        .map_err(|e| AppError::InputError(e.to_string()))?;
    for &m in modifiers.iter().rev() {
        enigo
            .key(m, Direction::Release)
            .map_err(|e| AppError::InputError(e.to_string()))?;
    }
    Ok(())
}

/// Moves the mouse to absolute screen coordinates and clicks.
pub fn click_at(x: i32, y: i32, button: &str) -> Result<(), AppError> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| AppError::InputError(e.to_string()))?;
    enigo
        .move_mouse(x, y, Coordinate::Abs)
        .map_err(|e| AppError::InputError(e.to_string()))?;
    enigo
        .button(parse_button_pal(button), Direction::Click)
        .map_err(|e| AppError::InputError(e.to_string()))
}

/// Types `text` into the currently focused element.
pub fn type_text(text: &str) -> Result<(), AppError> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| AppError::InputError(e.to_string()))?;
    enigo
        .text(text)
        .map_err(|e| AppError::InputError(e.to_string()))
}

/// Sends a key combination such as `ctrl+s` or `enter`.
pub fn send_keys(keys: &str) -> Result<(), AppError> {
    let (modifiers, final_key) = parse_combo_pal(keys)?;
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| AppError::InputError(e.to_string()))?;
    send_combo_pal(&mut enigo, &modifiers, final_key)
}
