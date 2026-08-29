use crate::{
    application::interface::{
        editing_session_repository::error::EditingSessionRepositoryError, image_segmenter::error::SegmenterRuntimeError, segmenter_input_image_storage::error::SegmenterInputImageStorageError,
    }, domain::repository::{
        session_repository::error::SessionRepositoryError,
    }, parent_error,
};

parent_error!(
    pub enum SegmentUseCaseError {
        SessionRepository(SessionRepositoryError),
        EditingSessionRepository(EditingSessionRepositoryError),
        Segmenter(SegmenterRuntimeError),
        SegmenterInputStorage(SegmenterInputImageStorageError),
    }
);
