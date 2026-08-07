use crate::leaf_error;

leaf_error!(
    pub enum RenderedCacheError {
        ImageNotFound => "IMAGE_NOT_FOUND",
    }
);