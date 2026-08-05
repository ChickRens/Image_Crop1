use crate::{domain::{entity::session::error::SessionError, repository::{image_repository::error::ImageRepositoryError, session_repository::error::SessionRepositoryError}}, parent_error};

parent_error!(
    pub enum GetImageUseCaseError {
        ImageRepository(ImageRepositoryError),
        SessionRepository(SessionRepositoryError),
        Session(SessionError),
    }
);

