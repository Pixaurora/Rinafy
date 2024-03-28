use std::path::PathBuf;

use crate::color_shifting::shift_colors;
use crate::errors::OperationError;
use image::io::Reader as ImageReader;

pub mod color_shifting;
pub mod errors;
pub mod exporting;

pub fn rinafy_file(in_file: PathBuf, out_file: PathBuf) -> Result<(), OperationError> {
    let image = ImageReader::open(in_file)?.decode()?;

    exporting::save_frames(shift_colors(&image, 45), out_file)
}
