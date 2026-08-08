use crate::{application::{interface::{editing_session_repository::error::EditingSessionRepositoryError, image_segmenter::error::SegmenterError}, types::editing_session::error::EditingSessionError}, domain::repository::{original_image_repository::error::OriginalImageRepositoryError, session_repository::error::SessionRepositoryError}, parent_error};

parent_error!(
    pub enum UndoUseCaseError {
        ImageRepository(OriginalImageRepositoryError),
        SessionRepository(SessionRepositoryError),
        Segmenter(SegmenterError),
        EditingSessionRepository(EditingSessionRepositoryError),
        EditingSession(EditingSessionError),
    }
);
