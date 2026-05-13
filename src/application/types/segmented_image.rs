use crate::domain::entity::image::Image;
use ndarray::Array2;

pub struct SegmentedImage {
    image: Image,
    mask: Option<Array2<f32>>,
}

impl SegmentedImage {
    pub fn new(image: Image) -> Self {
        Self { image, mask: None }
    }

    pub fn with_mask(image: Image, mask: Array2<f32>) -> Self {
        Self { image, mask: Some(mask) }
    }

    pub fn into_image(self) -> Image {
        self.image
    }

    pub fn mask(&self) -> Option<&Array2<f32>> {
        self.mask.as_ref()
    }
}
