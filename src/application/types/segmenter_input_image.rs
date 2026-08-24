use crate::domain::entity::image::Image;

pub struct SegmenterInputImage {
    image: Image
}

impl SegmenterInputImage {
    pub fn new(image: Image) -> Self {
        Self { image }
    }

    pub fn into_image(self) -> Image {
        self.image
    }
}