use crate::domain::{entity::image::Image, value_object::image_id::image_id::ImageId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginalImage {
    image: Image,
}

impl OriginalImage {
    pub fn new(image: Image) -> Self {
        Self { image }
    }

    pub fn into_image(self) -> Image {
        self.image
    }

    pub fn image(&self) -> &Image {
        &self.image
    }

    pub fn image_id(&self) -> ImageId {
        *self.image.image_id()
    }
}
