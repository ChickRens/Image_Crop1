use crate::leaf_error;

leaf_error!(
    pub enum ImageRepositoryError {
        ImageNotFound => "IMAGE_NOT_FOUND",
    }
);