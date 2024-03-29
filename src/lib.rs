use std::path::PathBuf;

use crate::color_shifting::shift_colors;
use crate::errors::RinafyResult;
use crate::mask::Mask;
use image::io::Reader as ImageReader;

pub mod color_shifting;
pub mod errors;
pub mod exporting;
pub mod mask;

pub fn rinafy_file(in_file: &PathBuf, out_file: &PathBuf, mask: &Mask) -> RinafyResult<()> {
    let image = ImageReader::open(in_file)?.decode()?;

    let frames = shift_colors(&image, mask, 45);

    exporting::save_frames(frames, out_file)
}
