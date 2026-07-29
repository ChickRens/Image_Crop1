use crate::domain::errors::traits::{Cause, Code};

#[derive(Debug, PartialEq, Eq)]
pub enum LoadingErrors {
    InvalidSize,
    UnsupportedFormat(String),
    CorruptedImage(String),
}

impl Code for LoadingErrors {
    fn code(&self) -> &str {
        match self {
            Self::InvalidSize => "INVALID_SIZE",
            Self::UnsupportedFormat(_) => "INVALID_FORMAT",
            Self::CorruptedImage(_) => "CORRUPTED",
        }
    }
}

impl Cause for LoadingErrors {
    fn cause(&self) -> Option<&str> {
        match self {
            Self::InvalidSize => None,
            Self::UnsupportedFormat(err) => Some(err),
            Self::CorruptedImage(err) => Some(err),
        }
    }
}