# src/core/vision_core.rs

## `pub struct MatchResult_core`

*Line 8 · struct*

Result of a successful template match.

---

## `pub fn find_template( screenshot: &RgbImage, template: &RgbImage, threshold: Option<f32>, ) -> Option<MatchResult_core>`

*Line 19 · fn*

Searches `screenshot` for `template` using NCC.
Returns the best match centre if its score meets `threshold`.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=7" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
