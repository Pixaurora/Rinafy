use image::DynamicImage;

use crate::mask::Mask;

pub fn color_in_mask(image: &DynamicImage, mask: &Mask, hue: i32) -> DynamicImage {
    image.huerotate(hue)
}

pub fn shift_colors(image: &DynamicImage, mask: &Mask, frame_count: i32) -> Vec<DynamicImage> {
    let hue_step = 360 / frame_count;

    (0..frame_count)
        .map(|frame_number| frame_number * hue_step)
        .map(|hue| color_in_mask(image, mask, hue))
        .collect()
}
