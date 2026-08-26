use crate::{application::types::segmenter_input_image::SegmenterInputImage, domain::entity::image::Image};

pub trait SegmenterInputImageGenerator {
    fn generate(&self, image: Image) -> SegmenterInputImage;
}