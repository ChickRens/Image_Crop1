use crate::domain::entity::image::Image;

pub struct CompletedImage {
    image: Image,
}

impl CompletedImage {
    pub fn new(image: Image) -> Self {
        Self { image }
    }

    pub fn into_image(self) -> Image {
        self.image
    }
}
