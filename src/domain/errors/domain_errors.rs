use crate::domain::errors::{
    image_errors::ImageErrors,
    session_errors::SessionErrors,
    traits::{Cause, Code},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainErrors {
    ImageError(ImageErrors),
    SessionError(SessionErrors),
}

impl Code for DomainErrors {
    fn code(&self) -> &str {
        match self {
            Self::ImageError(err) => err.code(),
            Self::SessionError(err) => err.code(),
        }
    }
}

impl Cause for DomainErrors {
    fn cause(&self) -> Option<&str> {
        match self {
            Self::ImageError(err) => err.cause(),
            Self::SessionError(err) => err.cause()
        }
    }
}
