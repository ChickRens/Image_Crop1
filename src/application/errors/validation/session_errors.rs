use crate::application::errors::application_errors::Code;

pub enum SessionErrors {
    NoSession,
    TimeOut,
    ImageNotOwned
}

impl Code for SessionErrors {
    fn code(&self) -> &str {
        match self {
            Self::NoSession => "SESSION_IS_NONE",
            Self::TimeOut => "INVALID_SESSION",
            Self::ImageNotOwned => "NOT_OWNED_IMAGE",
        }
    }
}
