use crate::{
    application::{
        interface::{
            editing_session_repository::error::EditingSessionRepositoryError, image_segmenter::error::SegmenterRuntimeError,
        }, types::editing_session::error::EditingSessionError,
    }, domain::repository::{
        original_image_repository::error::OriginalImageRepositoryError,
        session_repository::error::SessionRepositoryError,
    }, parent_error,
};

parent_error!(
    pub enum RedoUseCaseError {
        ImageRepository(OriginalImageRepositoryError),
        SessionRepository(SessionRepositoryError),
        EditingSessionRepository(EditingSessionRepositoryError),
        Segmenter(SegmenterRuntimeError),
        EditingSession(EditingSessionError),
    }
);
