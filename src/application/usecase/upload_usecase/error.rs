use crate::{
    application::interface::{
        editing_session_repository::error::EditingSessionRepositoryError,
        image_loader::error::LoadingError, image_segmenter::error::SegmenterError,
    },
    domain::repository::{
        original_image_repository::error::OriginalImageRepositoryError,
        session_repository::error::SessionRepositoryError,
    },
    parent_error,
};

parent_error!(
    pub enum UploadUseCaseError {
        ImageRepository(OriginalImageRepositoryError),
        SessionRepository(SessionRepositoryError),
        Segmenter(SegmenterError),
        EditingSessionRepository(EditingSessionRepositoryError),
        Loader(LoadingError),
    }
);
