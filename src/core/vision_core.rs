/// Template matching via normalised cross-correlation (NCC).
use image::RgbImage;
use crate::state::limits::{DEFAULT_CONFIDENCE, NCC_STD_FLOOR};
use crate::state::sizes::{CENTER_DIVISOR, RGB_CHANNELS};

/// Result of a successful template match.
#[allow(non_camel_case_types)]
pub struct MatchResult_core {
    /// Centre X relative to the screenshot.
    pub x: u32,
    /// Centre Y relative to the screenshot.
    pub y: u32,
    /// NCC score in range [0, 1].
    pub confidence: f32,
}

/// Searches `screenshot` for `template` using NCC.
/// Returns the best match centre if its score meets `threshold`.
pub fn find_template(
    screenshot: &RgbImage,
    template: &RgbImage,
    threshold: Option<f32>,
) -> Option<MatchResult_core> {
    let threshold = threshold.unwrap_or(DEFAULT_CONFIDENCE);
    let (sw, sh) = screenshot.dimensions();
    let (tw, th) = template.dimensions();

    if tw > sw || th > sh {
        return None;
    }

    let t_pixels: Vec<f32> = template
        .pixels()
        .flat_map(|p| p.0.iter().map(|&c| c as f32))
        .collect();
    let t_mean: f32 = t_pixels.iter().sum::<f32>() / t_pixels.len() as f32;
    let t_std: f32 = variance(&t_pixels, t_mean).sqrt();

    let mut best_score = f32::NEG_INFINITY;
    let mut best_x = 0u32;
    let mut best_y = 0u32;

    for y in 0..=(sh - th) {
        for x in 0..=(sw - tw) {
            let score = ncc_at(screenshot, &t_pixels, x, y, tw, th, t_mean, t_std);
            if score > best_score {
                best_score = score;
                best_x = x;
                best_y = y;
            }
        }
    }

    if best_score >= threshold {
        Some(MatchResult_core {
            x: best_x + tw / CENTER_DIVISOR,
            y: best_y + th / CENTER_DIVISOR,
            confidence: best_score,
        })
    } else {
        None
    }
}

fn ncc_at(
    screen: &RgbImage,
    t_pixels: &[f32],
    ox: u32,
    oy: u32,
    tw: u32,
    th: u32,
    t_mean: f32,
    t_std: f32,
) -> f32 {
    let n = (tw * th * RGB_CHANNELS) as f32;

    let r_pixels: Vec<f32> = (oy..oy + th)
        .flat_map(|y| {
            (ox..ox + tw)
                .flat_map(move |x| screen.get_pixel(x, y).0.iter().map(|&c| c as f32).collect::<Vec<_>>())
        })
        .collect();

    let r_mean: f32 = r_pixels.iter().sum::<f32>() / n;
    let r_std: f32 = variance(&r_pixels, r_mean).sqrt();

    if r_std < NCC_STD_FLOOR || t_std < NCC_STD_FLOOR {
        return 0.0;
    }

    r_pixels
        .iter()
        .zip(t_pixels.iter())
        .map(|(&r, &t)| (r - r_mean) * (t - t_mean))
        .sum::<f32>()
        / (n * r_std * t_std)
}

fn variance(pixels: &[f32], mean: f32) -> f32 {
    pixels.iter().map(|&v| (v - mean).powi(CENTER_DIVISOR as i32)).sum::<f32>() / pixels.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{imageops, Rgb, RgbImage};

    fn patterned_image(width: u32, height: u32) -> RgbImage {
        let mut img = RgbImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                img.put_pixel(
                    x,
                    y,
                    Rgb([
                        ((x * 17 + y * 3) % 251) as u8,
                        ((x * 7 + y * 19 + 11) % 241) as u8,
                        ((x * 13 + y * 5 + 23) % 239) as u8,
                    ]),
                );
            }
        }
        img
    }

    #[test]
    fn finds_exact_template_center() {
        let screenshot = patterned_image(20, 16);
        let template = imageops::crop_imm(&screenshot, 6, 7, 4, 3).to_image();

        let found = find_template(&screenshot, &template, Some(0.999)).expect("expected match");

        assert_eq!(found.x, 8);
        assert_eq!(found.y, 8);
        assert!(found.confidence >= 0.999);
    }

    #[test]
    fn rejects_threshold_above_perfect_match() {
        let screenshot = patterned_image(18, 12);
        let template = imageops::crop_imm(&screenshot, 5, 4, 3, 3).to_image();

        assert!(find_template(&screenshot, &template, Some(1.01)).is_none());
    }

    #[test]
    fn rejects_template_larger_than_screenshot() {
        let screenshot = patterned_image(4, 4);
        let template = patterned_image(5, 5);

        assert!(find_template(&screenshot, &template, Some(0.8)).is_none());
    }

    #[test]
    fn rejects_uniform_template_regions() {
        let screenshot = RgbImage::from_pixel(8, 8, Rgb([255, 255, 255]));
        let template = RgbImage::from_pixel(2, 2, Rgb([255, 255, 255]));

        assert!(find_template(&screenshot, &template, Some(0.1)).is_none());
    }
}
