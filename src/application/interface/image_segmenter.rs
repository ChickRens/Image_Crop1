use crate::application::errors::segmentation_error::SegmentationErrors;
use crate::application::types::editing_session::CommonEditingSession;
use crate::application::types::segmented_image::SegmentedImage;
use crate::domain::entity::image::Image;
use crate::domain::value_object::point::Point;

pub trait ImageSegmenter {
    type StaticContext;
    type InferenceContext;

    fn prepare_inference_context(
        &self,
        image: &Image,
    ) -> Result<Self::InferenceContext, SegmentationErrors>;
    fn prepare_static_context(
        &self,
        image: &Image,
    ) -> Result<Self::StaticContext, SegmentationErrors>;
    fn rebuild(
        &self,
        original_image: &Image,
        static_context: &Self::StaticContext,
        input_points: &[Point],
    ) -> Result<(Self::InferenceContext, SegmentedImage), SegmentationErrors>;
    fn segment(
        &self,
        original_image: &Image,
        static_context: &Self::StaticContext,
        inference_context: &Self::InferenceContext,
        input_points: &[Point],
    ) -> Result<(Self::InferenceContext, SegmentedImage), SegmentationErrors>;
}
