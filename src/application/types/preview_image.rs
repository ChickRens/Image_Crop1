use crate::domain::{
    entity::image::Image,
    value_object::{
        image_data::ImageData, image_id::image_id::ImageId, image_size::image_size::ImageSize,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewImage {
    image: Image,
}

impl PreviewImage {
    pub fn new(image: Image) -> Self {
        Self { image }
    }

    pub fn image_id(&self) -> ImageId {
        *self.image.image_id()
    }

    pub fn into_data(self) -> (ImageData, ImageId, ImageSize) {
        self.image.into_data()
    }
}
