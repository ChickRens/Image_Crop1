use crate::domain::entity::image::Image;

pub struct PreviewImage {
    image: Image
}

impl PreviewImage {
    pub fn new(image: Image) -> Self {
        Self { image }
    }

    pub fn into_image(self) -> Image {
        self.image
    }
}