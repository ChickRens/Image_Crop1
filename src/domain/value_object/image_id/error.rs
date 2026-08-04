use crate::domain::errors::traits::{Cause, Code};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageIdError {
    InvalidImageId
}

impl Code for ImageIdError {
    fn code(&self) -> &str {
        match self {
            Self::InvalidImageId => "INVALID_IMAGE_ID"
        }
    }
}

impl Cause for ImageIdError {
    fn cause(&self) -> Option<&str> {
        match self {
            Self::InvalidImageId => None
        }
    }
}