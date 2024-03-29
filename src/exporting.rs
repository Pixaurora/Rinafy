use std::fs::File;
use std::path::PathBuf;
use std::thread;

use crate::errors::OperationError;
use crate::errors::RinafyResult;
use gifski::progress::NoProgress;
use gifski::Repeat;
use gifski::Settings;
use image::DynamicImage;
use image::Rgba;
use imgref::ImgVec;
use rgb::RGBA8;

fn image_to_imgref(image: &DynamicImage) -> ImgVec<RGBA8> {
    let width = image.width() as usize;
    let height = image.height() as usize;

    let pixels: Vec<Rgba<u8>> = image
        .clone()
        .into_rgba8()
        .pixels()
        .map(ToOwned::to_owned)
        .collect();

    ImgVec::new(pixels, width, height).map_buf(|pixels| {
        pixels
            .iter()
            .map(|pixel| {
                let [r, g, b, a] = pixel.0;
                RGBA8::new(r, g, b, a)
            })
            .collect()
    })
}

pub fn save_frames(frames: Vec<DynamicImage>, out_file: &PathBuf) -> RinafyResult<()> {
    let gifski_settings = Settings {
        width: None,
        height: None,
        quality: 100,
        fast: false,
        repeat: Repeat::Infinite,
    };

    let (collector, writer) = gifski::new(gifski_settings)?;

    let length_of_one_frame = 0.1;

    let handle = thread::spawn(move || {
        for (i, frame) in frames.iter().enumerate() {
            collector.add_frame_rgba(i, image_to_imgref(frame), length_of_one_frame * i as f64)?;
        }

        Ok(())
    });

    let buffer = File::create(out_file)?;
    writer.write(buffer, &mut NoProgress {})?;

    handle
        .join()
        .or_else(|_| Err(OperationError::Other(String::from("thread somehow died?"))))?
}
