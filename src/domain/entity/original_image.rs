use crate::domain::entity::image::Image;

pub struct OriginalImage {
    image: Image
}

impl OriginalImage {
    pub fn new(image: Image) -> Self {
        Self { image }
    }

    pub fn into_image(self) -> Image {
        self.image
    }
}