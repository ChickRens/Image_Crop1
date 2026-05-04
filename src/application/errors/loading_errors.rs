use crate::application::errors::application_errors::Code;

pub enum LoadingErrors {
    InvalidSize,
    UnsupportedFormat,
    CorruptedImage,
}

impl Code for LoadingErrors {
    fn code(&self) -> &str {
        match self {
            Self::InvalidSize => "INVALID_SIZE",
            Self::UnsupportedFormat => "INVALID_FORMAT",
            Self::CorruptedImage => "CORRUPTED",
        }
    }
}
