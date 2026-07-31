use crate::domain::errors::traits::Code;

pub enum SessionRepositoryError {
    SessionNotFound,
}

impl Code for SessionRepositoryError {
    fn code(&self) -> &str {
        match self {
            Self::SessionNotFound => "SESSION_NOT_FOUND"
        }
    }
}