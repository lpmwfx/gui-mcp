# src/adapter/app_adp/clipboard_adp.rs

## `pub fn select_all_adp(window_title: &str) -> Result<(), AppError>`

*Line 6 · fn*

Selects all text in the named window's edit control.

---

## `pub fn copy_adp(window_title: &str) -> Result<(), AppError>`

*Line 12 · fn*

Copies the current selection in the named window.

---

## `pub fn cut_adp(window_title: &str) -> Result<(), AppError>`

*Line 18 · fn*

Cuts the current selection in the named window.

---

## `pub fn paste_adp(window_title: &str) -> Result<(), AppError>`

*Line 24 · fn*

Pastes clipboard content into the named window's edit control.

---

