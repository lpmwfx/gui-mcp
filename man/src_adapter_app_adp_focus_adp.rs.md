# src/adapter/app_adp/focus_adp.rs

## `pub fn focused_type_text(window_title: &str, text: &str) -> Result<(), AppError>`

*Line 6 · fn*

Finds the named window and types text into it via WM_CHAR.

---

## `pub fn focused_send_keys(window_title: &str, keys: &str) -> Result<(), AppError>`

*Line 12 · fn*

Finds the named window and sends a key combination via WM_KEYDOWN/UP.

---

## `pub fn click_at_adp(window_title: &str, x: i32, y: i32, button: &str) -> Result<(), AppError>`

*Line 18 · fn*

Finds the named window and clicks at absolute client coordinates.

---

