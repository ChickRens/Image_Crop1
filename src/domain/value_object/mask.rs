use crate::domain::value_object::image_size::ImageSize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Mask {
    size: ImageSize,
}

impl Mask {
    pub fn new(size: ImageSize) -> Self {
        Self { size }
    }

    pub fn mask_size(&self) -> &ImageSize {
        &self.size
    }
}
