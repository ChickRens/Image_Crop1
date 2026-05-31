use crate::application::errors::segmentation_error::SegmentationErrors;
use crate::application::types::editing_session::CommonEditingSession;
use crate::application::types::segmented_image::SegmentedImage;
use crate::domain::entity::image::Image;

pub trait ImageSegmenter {
    type StaticContext;
    type InferenceContext;

    fn segment(
        &mut self,
        original_image: &Image,
        editing_session: &mut CommonEditingSession<Self::StaticContext, Self::InferenceContext>,
    ) -> Result<SegmentedImage, SegmentationErrors>;
}

pub trait ImageSegmenterPreparing {
    type SegmentationContext;

    fn prepare(&mut self, image: Image) -> Result<Self::SegmentationContext, SegmentationErrors>;
}
