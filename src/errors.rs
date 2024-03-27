use std::io::Error as IOError;

use image::ImageError;

pub enum OperationError {
    Io(IOError),
    Image(ImageError),
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
