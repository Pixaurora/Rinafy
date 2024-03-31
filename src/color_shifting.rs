use crate::{conversion::RinafyConvert, mask::Mask};
use color_art::{blend, BlendMode, Color};
use image::{DynamicImage, RgbaImage};

pub fn color_pixel(image: &mut RgbaImage, (x, y): (u32, u32), hue: f64, opacity: f64) {
    let base_color = image.get_pixel(x, y).convert();

    let effect_color = Color::from_hsv(hue, 1.0, 1.0).expect("Should be in bounds");

    // Mix with the base color before blending, since blending doesn't currently account for true opacity
    let effect_color = effect_color.mix_with(&base_color, 1.0 - opacity);

    let new_color = blend(&base_color, &effect_color, BlendMode::SoftLight).convert();

    image.put_pixel(x, y, new_color);
}

pub fn color_in_mask(image: &DynamicImage, mask: &Mask, hue: f64) -> DynamicImage {
    let mut frame = image.clone().into_rgba8();

    for y in 0..frame.height() {
        for x in 0..frame.width() {
            let effect_intensity = mask.intensity((x, y));

            if effect_intensity != 0 {
                let effect_intensity = effect_intensity as f64 / 256.0;
                color_pixel(&mut frame, (x, y), hue, effect_intensity);
            }
        }
    }

    DynamicImage::ImageRgba8(frame)
}

pub fn shift_colors(image: &DynamicImage, mask: &Mask, frame_count: u32) -> Vec<DynamicImage> {
    let hue_step = 360.0 / frame_count as f64;

    (0..frame_count)
        .map(|frame_number| frame_number as f64 * hue_step)
        .map(|hue| color_in_mask(image, mask, hue))
        .collect()
}
