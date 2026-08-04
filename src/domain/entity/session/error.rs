use crate::domain::errors::traits::{Cause, Code};

pub enum SessionError {
    ImageNotOwned
}

impl Code for SessionError {
    fn code(&self) -> &str {
        match self {
            Self::ImageNotOwned => "IMAGE_NOT_OWNED"
        }
    }
}

impl Cause for SessionError {
    fn cause(&self) -> Option<&str> {
        match self {
            Self::ImageNotOwned => None
        }
    }
}