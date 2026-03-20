# src/pal/window_pal/mod.rs

## `pub(crate) mod gdi_pal;`

*Line 4 · mod*

GDI palette abstraction and color management utilities for graphics operations.

---

## `pub fn find_window_by_partial_title(partial: &str) -> Result<u64, AppError>`

*Line 63 · fn*

Finds the first visible window whose title contains `partial` (case-insensitive).

---

## `pub fn get_window_rect(hwnd: u64) -> Result<(i32, i32, i32, i32), AppError>`

*Line 92 · fn*

Returns the screen rect (left, top, right, bottom) for `hwnd`.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=7" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
