use crate::{
    application::errors::application_errors::Code,
    domain::errors::{image_errors::ImageErrors, session_errors::SessionErrors},
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
