use crate::{application::{interface::{editing_session_repository::error::EditingSessionRepositoryError, image_segmenter::error::SegmenterRuntimeError, segmenter_input_image_storage::error::SegmenterInputImageStorageError}, types::editing_session::error::EditingSessionError}, domain::repository::{original_image_repository::error::OriginalImageRepositoryError, session_repository::error::SessionRepositoryError}, parent_error};

parent_error!(
    pub enum SegmentServiceError {
        SessionRepository(SessionRepositoryError),
        ImageRepository(OriginalImageRepositoryError),
        SegmenterInputStorage(SegmenterInputImageStorageError),
        EditingSessionRepository(EditingSessionRepositoryError),
        Segmenter(SegmenterRuntimeError),
        EditingSession(EditingSessionError),
    }
);