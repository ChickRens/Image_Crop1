use crate::domain::errors::traits::{Cause, Code};

#[derive(Debug, PartialEq, Eq)]
pub enum SessionErrors {
    TimeOut,
    ImageNotOwned,
}

impl Code for SessionErrors {
    fn code(&self) -> &str {
        match self {
            Self::TimeOut => "INVALID_SESSION",
            Self::ImageNotOwned => "NOT_OWNED_IMAGE",
        }
    }
}

impl Cause for SessionErrors {
    fn cause(&self) -> Option<&str> {
        match self {
            Self::ImageNotOwned => None,
            Self::TimeOut => None,
        }
    }
}