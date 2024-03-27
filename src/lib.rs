use std::path::PathBuf;

use crate::color_shifting::shift_colors;
use crate::errors::OperationError;
use image::io::Reader as ImageReader;

pub mod color_shifting;
pub mod errors;

pub fn rinafy_file(in_file: PathBuf, out_file: PathBuf) -> Result<(), OperationError> {
    let img = ImageReader::open(in_file)?.decode()?;

    let _frames = shift_colors(&img, 45);

    img.save(out_file)?;

    Ok(())
}
