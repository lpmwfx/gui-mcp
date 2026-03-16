# src/pal/input_pal/vk_pal.rs

## `pub(crate) static VK_MAP_PAL: &[(&str, u16)] = &[ ("ctrl",      0x11), ("control",   0x11), ("alt",       0x12), ("shift",     0x10), ("win",       0x5B), ("meta",      0x5B), ("super",     0x5B),`

*Line 17 · const*

Maps virtual key names to their corresponding key codes for the PAL keyboard layout. Each tuple contains a key name string and its associated u16 key code.

---

## `pub(crate) fn parse_vk_pal(name: &str) -> Option<u16>`

*Line 57 · fn*

Parses a virtual key code from a key name, looking it up in VK_MAP_PAL or treating a single character as its ASCII value.

---

## `pub(crate) fn parse_button_msgs_pal(btn: &str) -> (u32, u32)`

*Line 68 · fn*

Returns Windows message IDs for button down and up events based on the button name, defaulting to left button if the name is not recognized.

---

## `pub(crate) fn parse_combo_vk_pal(keys: &str) -> Result<(Vec<u16>, u16), AppError>`

*Line 82 · fn*

Parses a keyboard combo string (e.g., "Ctrl+Shift+A") into modifier keys and a final key.
Splits on '+' with the last component as the final key and preceding components as modifier keys.
Returns a tuple of (modifier virtual keys, final virtual key) or an error if any key is invalid.

---

## `pub(crate) fn post_combo_pal(hwnd_val: HWND, mod_vks: &[u16], final_vk: u16)`

*Line 108 · fn*

Simulates pressing a keyboard combination with modifiers by posting key down and key up messages to the target window in the correct sequence.

---

