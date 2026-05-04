use crate::{application::types::image::Image, domain::value_object::image_size::ImageSize};

pub struct LoadedImage {
    image: Image,
    size: ImageSize,
}

impl LoadedImage {
    pub fn new(image: Image, image_size: ImageSize) -> Self {
        Self {
            image,
            size: image_size,
        }
    }

    pub fn into_image_and_size(self) -> (Image, ImageSize) {
        (self.image, self.size)
    }
}
