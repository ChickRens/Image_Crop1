use crate::domain::errors::traits::{Cause, Code};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageSizeError {
    LongHeight,
    LongWidth,
    ShortHeight,
    ShortWidth,
}

impl Code for ImageSizeError {
    fn code(&self) -> &str {
        match self {
            Self::LongHeight => "LONG_HEIGHT",
            Self::LongWidth => "LONG_WIDTH",
            Self::ShortHeight => "SHORT_HEIGHT",
            Self::ShortWidth => "SHORT_WIDTH"
        }
    }
}

impl Cause for ImageSizeError {
    fn cause(&self) -> Option<&str> {
        match self {
            Self::LongHeight => None,
            Self::LongWidth => None,
            Self::ShortHeight => None,
            Self::ShortWidth => None
        }
    }
}