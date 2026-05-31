use crate::domain::value_object::image_data::ImageData;

pub struct SegmentedImage {
    image: ImageData,
}

impl SegmentedImage {
    pub fn new(image: ImageData) -> Self {
        Self { image }
    }

    pub fn into_image(self) -> ImageData {
        self.image
    }
}
