use crate::{
    domain::value_object::{image_id::error::ImageIdError, image_size::error::ImageSizeError},
    parent_error,
};

parent_error!(
    pub enum ImageError {
        ImageId(ImageIdError),
        ImageSize(ImageSizeError),
    }
);
