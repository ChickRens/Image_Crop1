use crate::{application::{interface::image_segmenter::error::SegmenterError, types::segmented_image::SegmentedImage}, domain::{entity::image::image::Image, value_object::point::Point}};

pub trait ImageSegmenter {
    type StaticContext;
    type InferenceContext;

    fn prepare_inference_context(
        &self,
        image: &Image,
    ) -> Result<Self::InferenceContext, SegmenterError>;
    fn prepare_static_context(
        &self,
        image: &Image,
    ) -> Result<Self::StaticContext, SegmenterError>;
    fn rebuild(
        &self,
        original_image: &Image,
        static_context: &Self::StaticContext,
        input_points: &[Point],
    ) -> Result<(Self::InferenceContext, SegmentedImage), SegmenterError>;
    fn segment(
        &self,
        original_image: &Image,
        static_context: &Self::StaticContext,
        inference_context: &Self::InferenceContext,
        input_points: &[Point],
    ) -> Result<(Self::InferenceContext, SegmentedImage), SegmenterError>;
}
