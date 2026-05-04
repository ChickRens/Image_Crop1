use crate::{application::types::image::Image, domain::value_object::image_size::ImageSize};

pub struct SegmentedImage {
    image: Image,
    size: ImageSize,
}

impl SegmentedImage {
    pub fn new(image: Image, image_size: ImageSize) -> Self {
        Self {
            image,
            size: image_size,
        }
    }

    pub fn image(&self) -> &Image {
        &self.image
    }

    pub fn size(&self) -> &ImageSize {
        &self.size
    }

    pub fn into_image_and_size(self) -> (Image, ImageSize) {
        (self.image, self.size)
    }
}
