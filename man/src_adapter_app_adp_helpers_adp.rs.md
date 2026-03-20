# src/adapter/app_adp/helpers_adp.rs

## `pub(crate) fn find_and_capture_adp(window_title: &str) -> Result<(u64, RgbImage), AppError>`

*Line 8 · fn*

Finds a window by partial title and captures its bitmap.

---

## `pub(crate) fn decode_template_adp(template_base64: &str) -> Result<RgbImage, AppError>`

*Line 15 · fn*

Decodes a base64 PNG/JPEG template into an RgbImage.

---

## `pub(crate) fn after_screenshot_adp(hwnd: u64) -> Result<String, AppError>`

*Line 24 · fn*

Captures `hwnd` after an action and returns it as a base64 PNG string.

---

## `pub(crate) fn encode_png(img: &RgbImage) -> Result<Vec<u8>, AppError>`

*Line 31 · fn*

Encodes an RgbImage to raw PNG bytes.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=7" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
