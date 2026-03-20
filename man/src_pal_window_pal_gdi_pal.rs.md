# src/pal/window_pal/gdi_pal.rs

## `pub(crate) fn gdi_capture_raw_pal(hwnd: u64) -> Result<(Vec<u8>, i32, i32), AppError>`

*Line 20 · fn*

Captures raw BGRA pixels from `hwnd` using PrintWindow.
Works even when the window is behind other windows or on another virtual desktop.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=7" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
