use crate::application::errors::application_errors::Code;

pub enum RepositoryErrors {
    ImageNotFound,
    SessionNotFound
}

impl Code for RepositoryErrors {
    fn code(&self) -> &str {
        match self {
            Self::ImageNotFound => "IMAGE_NOT_FOUND",
            Self::SessionNotFound => "SESSION_NOT_FOUND",
        }
    }
}
