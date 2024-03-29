use std::{fmt::Display, path::PathBuf, str::FromStr};

use crate::errors::{OperationError, RinafyResult};
use image::{io::Reader as ImageReader, RgbaImage};

pub enum Mask {
    AlphaBased(AlphaBasedMask),
    AllowAll,
}

pub struct AlphaBasedMask {
    mask_image: RgbaImage,
}

#[derive(Clone, Debug)]
pub enum MaskKind {
    NoMask,
    FromFile(PathBuf),
}

impl AlphaBasedMask {
    pub fn new(mask_image: RgbaImage) -> Self {
        AlphaBasedMask { mask_image }
    }

    pub fn get_intensity(&self, (x, y): (u32, u32)) -> u8 {
        self.mask_image
            .get_pixel_checked(x, y)
            .map(|color| color.0[3])
            .unwrap_or(0)
    }
}

impl Mask {
    pub fn intensity(&self, point: (u32, u32)) -> u8 {
        match self {
            Mask::AlphaBased(mask) => mask.get_intensity(point),
            Mask::AllowAll => 100,
        }
    }

    pub fn load(in_file: PathBuf) -> RinafyResult<Self> {
        let mask_image = ImageReader::open(in_file)?.decode()?;

        return Ok(Mask::AlphaBased(AlphaBasedMask::new(
            mask_image.into_rgba8(),
        )));
    }
}

impl MaskKind {
    pub fn create_mask(self) -> RinafyResult<Mask> {
        match self {
            MaskKind::NoMask => Ok(Mask::AllowAll),
            MaskKind::FromFile(in_file) => Mask::load(in_file),
        }
    }
}

impl FromStr for MaskKind {
    type Err = OperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "none" => MaskKind::NoMask,
            _ => MaskKind::FromFile(PathBuf::from(s)),
        })
    }
}

impl Display for MaskKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaskKind::NoMask => write!(f, "none"),
            MaskKind::FromFile(in_file) => write!(f, "{}", in_file.to_string_lossy()),
        }
    }
}
