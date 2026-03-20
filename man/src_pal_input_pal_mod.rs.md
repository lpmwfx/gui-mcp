# src/pal/input_pal/mod.rs

## `pub(crate) mod vk_pal;`

*Line 7 · mod*

Virtual-key code mapping and combo posting.

---

## `pub fn type_text(hwnd: u64, text: &str) -> Result<(), AppError>`

*Line 23 · fn*

Types text into `hwnd` via WM_CHAR messages. No foreground required.
Converts `\n` to `\r` for RichEdit compatibility.

---

## `pub fn send_keys(hwnd: u64, keys: &str) -> Result<(), AppError>`

*Line 45 · fn*

Sends a key combination to `hwnd` via WM_KEYDOWN/WM_KEYUP. No foreground required.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=7" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
