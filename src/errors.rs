use std::io::Error as IOError;

use gifski::Error as GifskiError;
use image::ImageError;

pub enum OperationError {
    Gifski(GifskiError),
    Io(IOError),
    Image(ImageError),
    Other(String),
}

impl From<IOError> for OperationError {
    fn from(value: IOError) -> Self {
        OperationError::Io(value)
    }
}

impl From<ImageError> for OperationError {
    fn from(value: ImageError) -> Self {
        if let ImageError::IoError(error) = value {
            OperationError::Io(error)
        } else {
            OperationError::Image(value)
        }
    }
}

impl From<GifskiError> for OperationError {
    fn from(value: GifskiError) -> Self {
        if let GifskiError::Io(error) = value {
            OperationError::Io(error)
        } else {
            OperationError::Gifski(value)
        }
    }
}
