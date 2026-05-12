use crate::application::errors::segmentation_error::SegmentationErrors;
use crate::domain::entity::image::Image;
use crate::application::types::segmented_image::SegmentedImage;
use crate::domain::value_object::point::Point;

pub trait ImageSegmenter {
    fn segment(&self, image:Image, points: Option<&Point>) -> Result<SegmentedImage,SegmentationErrors>;
}

pub trait ImageSegmenterPreparing {
    fn prepare(&mut self, image: Image) -> Result<(), SegmentationErrors>;
}