use crate::domain::entity::image::Image;
use crate::application::types::segmented_image::SegmentedImage;
use crate::domain::value_object::point::Point;

pub trait ImageSegmenter {
    fn segment(&self, image:Image, points: Option<&Point>) -> SegmentedImage;
}