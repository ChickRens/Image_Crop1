use crate::domain::errors::traits::{Cause, Code};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionErrors {
    InvalidSessionId,
}

impl Code for SessionErrors {
    fn code(&self) -> &str {
        match self {
            Self::InvalidSessionId => "INVALID_SESSION_ID",
        }
    }
}

impl Cause for SessionErrors {
    fn cause(&self) -> Option<&str> {
        match self {
            Self::InvalidSessionId => None
        }
    }
}