use crate::{
    application::{interface::image_segmenter::error::{SegmenterLoadingError, SegmenterRuntimeError}, types::segmented_image::SegmentedImage}, domain::{entity::image::Image, value_object::point::Point},
};

pub trait ImageSegmenter {
    type StaticContext;
    type InferenceContext;

    fn prepare_inference_context(
        &self,
        image: &Image,
    ) -> Result<Self::InferenceContext, SegmenterLoadingError>;
    fn prepare_static_context(&self, image: &Image) -> Result<Self::StaticContext, SegmenterLoadingError>;
    fn segment(
        &self,
        original_image: &Image,
        static_context: &Self::StaticContext,
        inference_context: &Self::InferenceContext,
        input_points: &[Point],
    ) -> Result<(Self::InferenceContext, SegmentedImage), SegmenterRuntimeError>;
}
