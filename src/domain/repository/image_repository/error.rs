use crate::domain::errors::traits::Code;

pub enum ImageRepositoryError {
    ImageNotFound
}

impl Code for ImageRepositoryError {
    fn code(&self) -> &str {
        match self {
            Self::ImageNotFound => "IMAGE_NOT_FOUND"
        }
    }
}