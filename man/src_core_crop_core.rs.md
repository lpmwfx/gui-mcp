# src/core/crop_core.rs

## `pub fn crop_region_core(img: &RgbImage, x: u32, y: u32, w: u32, h: u32) -> Option<RgbImage>`

*Line 6 · fn*

Crops a rectangular region from `img` at `(x, y)` with size `(w, h)`.
Returns `None` if the region extends beyond the image bounds.

---

