/// Pure image cropping -- extracts a rectangular region from an RgbImage.
use image::RgbImage;

/// Crops a rectangular region from `img` at `(x, y)` with size `(w, h)`.
/// Returns `None` if the region extends beyond the image bounds.
pub fn crop_region_core(img: &RgbImage, x: u32, y: u32, w: u32, h: u32) -> Option<RgbImage> {
    let (iw, ih) = img.dimensions();
    if x + w > iw || y + h > ih || w == 0 || h == 0 {
        return None;
    }
    Some(image::imageops::crop_imm(img, x, y, w, h).to_image())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crop_valid_region() {
        let img = RgbImage::from_pixel(100, 100, image::Rgb([128, 64, 32]));
        let cropped = crop_region_core(&img, 10, 20, 30, 40);
        assert!(cropped.is_some());
        let c = cropped.unwrap();
        assert_eq!(c.dimensions(), (30, 40));
    }

    #[test]
    fn crop_out_of_bounds_returns_none() {
        let img = RgbImage::from_pixel(50, 50, image::Rgb([0, 0, 0]));
        assert!(crop_region_core(&img, 40, 40, 20, 20).is_none());
    }

    #[test]
    fn crop_zero_size_returns_none() {
        let img = RgbImage::from_pixel(50, 50, image::Rgb([0, 0, 0]));
        assert!(crop_region_core(&img, 0, 0, 0, 10).is_none());
        assert!(crop_region_core(&img, 0, 0, 10, 0).is_none());
    }
}
