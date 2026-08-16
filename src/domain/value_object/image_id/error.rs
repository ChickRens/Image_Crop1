use crate::{common::traits::ErrorType::InvalidInput, leaf_error};

leaf_error!(
    pub enum ImageIdError {
        InvalidImageId => ("INVALID_IMAGE_ID", InvalidInput),
    }
);
