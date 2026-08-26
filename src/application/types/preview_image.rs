use crate::domain::{entity::image::Image, value_object::{image_data::ImageData, image_size::image_size::ImageSize}};

pub struct PreviewImage {
    image: Image
}

impl PreviewImage {
    pub fn new(image: Image) -> Self {
        Self { image }
    }

    pub fn image_data(&self) -> &ImageData {
        self.image.image_data()
    }

    pub fn image_size(&self) -> &ImageSize {
        self.image.image_size()
    }
}