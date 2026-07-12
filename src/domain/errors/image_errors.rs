use crate::application::errors::application_errors::Code;

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
