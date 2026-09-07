use crate::{domain::repository::completed_image_repository::error::CompletedImageRepositoryError, parent_error};

parent_error!(
    pub enum GetCompletedUseCaseError {
        CompletedRepository(CompletedImageRepositoryError),
    }
);
