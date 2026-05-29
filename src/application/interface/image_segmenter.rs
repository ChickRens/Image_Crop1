use crate::application::errors::segmentation_error::SegmentationErrors;
use crate::application::types::mask::Mask;
use crate::domain::entity::image::Image;

pub trait ImageSegmenter {
    type SegmentationInputs;

    fn segment(&mut self, request: Self::SegmentationInputs) -> Result<Mask, SegmentationErrors>;
}

pub trait ImageSegmenterPreparing {
    type SegmentationContext;

    fn prepare(&mut self, image: Image) -> Result<Self::SegmentationContext, SegmentationErrors>;
}
