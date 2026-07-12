use crate::application::errors::application_errors::Code;

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
