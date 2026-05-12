use crate::domain::entity::image::Image;

pub struct SegmentedImage {
    image: Image,
}

impl SegmentedImage {
    pub fn new(image: Image) -> Self {
        Self { image }
    }

    pub fn into_image(self) -> Image {
        self.image
    }
}
