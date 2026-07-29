use crate::domain::errors::traits::{Cause, Code};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageErrors {
    InvalidImageId,
    LongHeight,
    LongWidth,
    ShortHeight,
    ShortWidth,
}

impl Code for ImageErrors {
    fn code(&self) -> &str {
        match self {
            Self::InvalidImageId => "INVALID_IMAGE_ID",
            Self::LongHeight => "LONG_HEIGHT",
            Self::LongWidth => "LONG_WIDTH",
            Self::ShortHeight => "SHORT_HEIGHT",
            Self::ShortWidth => "SHORT_WIDTH",
        }
    }
}

impl Cause for ImageErrors {
    fn cause(&self) -> Option<&str> {
        match self {
            Self::InvalidImageId => None,
            Self::LongHeight => None,
            Self::LongWidth => None,
            Self::ShortHeight => None,
            Self::ShortWidth => None
        }
    }
}
