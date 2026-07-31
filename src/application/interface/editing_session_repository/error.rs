use crate::domain::errors::traits::Code;

pub enum EditingSessionRepositoryError {
    EditingSessionNotFound
}

impl Code for EditingSessionRepositoryError {
    fn code(&self) -> &str {
        match self {
            Self::EditingSessionNotFound => "EDITING_SESSION_NOT_FOUND"
        }
    }
}