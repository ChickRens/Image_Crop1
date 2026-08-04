use crate::domain::errors::traits::{Cause, Code};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionIdError {
    InvalidSessionId
}

impl Code for SessionIdError {
    fn code(&self) -> &str {
        match self {
            Self::InvalidSessionId => "INVALID_SESSION_ID"
        }
    }
}

impl Cause for SessionIdError {
    fn cause(&self) -> Option<&str> {
        match self {
            Self::InvalidSessionId => None
        }
    }
}