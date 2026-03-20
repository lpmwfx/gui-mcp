# src/pal/input_pal/clipboard_pal.rs

## `pub fn select_all_pal(hwnd: u64) -> Result<(), AppError>`

*Line 12 · fn*

Selects all text in the edit control of `hwnd`.

---

## `pub fn copy_pal(hwnd: u64) -> Result<(), AppError>`

*Line 31 · fn*

Copies the current selection to the clipboard.

---

## `pub fn cut_pal(hwnd: u64) -> Result<(), AppError>`

*Line 50 · fn*

Cuts the current selection to the clipboard.

---

## `pub fn paste_pal(hwnd: u64) -> Result<(), AppError>`

*Line 69 · fn*

Pastes clipboard content into the edit control.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=7" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
