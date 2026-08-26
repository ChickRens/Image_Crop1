use crate::{application::interface::image_segmenter::error::SegmenterLoadingError, domain::repository::original_image_repository::error::OriginalImageRepositoryError, parent_error};

parent_error!(
    pub enum PrepareSegmentUseCaseError {
        ImageRepository(OriginalImageRepositoryError),
        Segmenter(SegmenterLoadingError),
    }
);