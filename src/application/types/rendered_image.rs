use crate::domain::value_object::{image_data::ImageData, image_id::image_id::ImageId, image_size::image_size::ImageSize};

#[derive(Debug, PartialEq, Clone)]
pub struct RenderedImage {
    data: ImageData,
    id: ImageId,
    size: ImageSize,
}

impl RenderedImage {
    pub fn new(data: ImageData, id: ImageId, size: ImageSize) -> Self {
        Self { data, id, size }
    }

    pub fn image_id(&self) -> ImageId {
        self.id
    }

    pub fn into_data(self) -> (ImageData, ImageId, ImageSize) {
        (self.data, self.id, self.size)
    }
}