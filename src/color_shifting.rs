use image::DynamicImage;

pub fn shift_colors(image: &DynamicImage, frames: i32) -> Vec<DynamicImage> {
    let hue_step = 360 / frames;

    (0..frames)
        .map(|frame_count| image.clone().huerotate(frame_count * hue_step))
        .collect()
}
