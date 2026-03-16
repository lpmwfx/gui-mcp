# src/pal/window_pal/focus_pal.rs

## `pub fn focus_window_pal(hwnd: u64) -> Result<(), AppError>`

*Line 5 · fn*

Calls SetForegroundWindow + short sleep to ensure the window receives input.

---

