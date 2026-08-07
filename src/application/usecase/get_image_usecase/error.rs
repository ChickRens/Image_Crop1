use crate::{application::interface::rendered_image_cache::error::RenderedCacheError, parent_error};

parent_error!(
    pub enum GetImageUseCaseError {
        ImageCache(RenderedCacheError),
    }
);

