use std::fs::File;
use std::path::PathBuf;
use std::thread;

use crate::conversion::RinafyConvert;
use crate::errors::OperationError;
use crate::errors::RinafyResult;
use gifski::progress::NoProgress;
use gifski::Repeat;
use gifski::Settings;
use image::DynamicImage;

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
        for (i, frame) in frames.into_iter().enumerate() {
            collector.add_frame_rgba(i, frame.convert(), length_of_one_frame * i as f64)?;
        }

        Ok(())
    });

    let buffer = File::create(out_file)?;
    writer.write(buffer, &mut NoProgress {})?;

    handle
        .join()
        .or_else(|_| Err(OperationError::Other(String::from("thread somehow died?"))))?
}
