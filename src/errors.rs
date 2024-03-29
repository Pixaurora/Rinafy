use std::io::Error as IOError;

use gifski::Error as GifskiError;
use image::ImageError;
use quick_error::quick_error;
use std::fmt::Debug;

quick_error!(
    pub enum OperationError {
        Args(message: String) {
            display("{message}")
        }
        Gifski(error: GifskiError) {
            display("{}", error)
        }
        Io(error: IOError) {
            display("{}", error)
        }
        Image(error: ImageError) {
            display("{}", error)
        }
        Other(message: String) {
            display("{message}")
        }
    }
);

pub type RinafyResult<T> = Result<T, OperationError>;

impl Debug for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
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
