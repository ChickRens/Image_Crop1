use crate::application::errors::segmentation_error::SegmentationErrors;
use crate::application::types::segmented_image::SegmentedImage;
use crate::domain::entity::image::Image;

pub trait ImageSegmenter {
    type SegmentationInputs;

    fn segment(
        &mut self,
        request: Self::SegmentationInputs
    ) -> Result<SegmentedImage, SegmentationErrors>;
}

pub trait ImageSegmenterPreparing {
    fn prepare(&mut self, image: Image) -> Result<(), SegmentationErrors>;
}
