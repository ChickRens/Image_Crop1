use crate::{application::interface::{editing_session_repository::error::EditingSessionRepositoryError, image_segmenter::error::SegmenterError}, domain::repository::{image_repository::error::ImageRepositoryError, session_repository::error::SessionRepositoryError}, parent_error};

parent_error!(
    pub enum RedoUseCaseError {
        ImageRepository(ImageRepositoryError),
        SessionRepository(SessionRepositoryError),
        EditingSessionRepository(EditingSessionRepositoryError),
        Segmenter(SegmenterError),
    }
);
