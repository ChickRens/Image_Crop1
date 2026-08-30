use crate::{application::interface::{editing_session_repository::error::EditingSessionRepositoryError, image_segmenter::error::SegmenterRuntimeError, segmenter_input_image_storage::error::SegmenterInputImageStorageError}, domain::repository::original_image_repository::error::OriginalImageRepositoryError, parent_error};

parent_error!(
    pub enum SegmentServiceError {
        ImageRepository(OriginalImageRepositoryError),
        SegmenterInputStorage(SegmenterInputImageStorageError),
        EditingSessionRepository(EditingSessionRepositoryError),
        Segmenter(SegmenterRuntimeError),
    }
);