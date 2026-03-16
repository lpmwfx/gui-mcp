# src/adapter/app_adp/burst_adp.rs

## `pub fn screenshot_burst( window_title: &str, count: usize, ) -> Result<Vec<(String, u32, u32)>, AppError>`

*Line 9 · fn*

Captures `count` screenshots with a short delay between each.
Returns a vec of (base64_png, width, height) tuples.

---

