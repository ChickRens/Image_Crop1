use std::sync::Arc;

use crate::{application::{interface::image_segmenter::{error::SegmenterError, segmenter::ImageSegmenter}, types::segmented_image::SegmentedImage}, domain::{entity::image::image::Image, value_object::point::Point}, infrastructure::segmenter::{sam2::Sam2Segmenter, sam2_data::{SAM2InferenceContext, SAM2StaticContext}}};

#[derive(Clone)]
pub struct SharedSAM2Segmenter {
    segmenter: Arc<Sam2Segmenter>,
}

impl ImageSegmenter for SharedSAM2Segmenter {
    type StaticContext = SAM2StaticContext;
    type InferenceContext = SAM2InferenceContext;

    fn segment(
        &self,
        original_image: &Image,
        static_context: &Self::StaticContext,
        inference_context: &Self::InferenceContext,
        input_points: &[Point],
    ) -> Result<(Self::InferenceContext, SegmentedImage), SegmenterError> {
        self.segmenter.segment(
            original_image,
            static_context,
            inference_context,
            input_points,
        )
    }

    fn rebuild(
        &self,
        original_image: &Image,
        static_context: &Self::StaticContext,
        input_points: &[Point],
    ) -> Result<(Self::InferenceContext, SegmentedImage), SegmenterError> {
        self.segmenter
            .rebuild(original_image, static_context, input_points)
    }

    fn prepare_inference_context(
        &self,
        image: &Image,
    ) -> Result<Self::InferenceContext, SegmenterError> {
        self.segmenter.prepare_inference_context(image)
    }

    fn prepare_static_context(
        &self,
        image: &Image,
    ) -> Result<Self::StaticContext, SegmenterError> {
        self.segmenter.prepare_static_context(image)
    }
}

impl SharedSAM2Segmenter {
    pub fn new(model_dir: &str) -> Result<Self, SegmenterError> {
        // Self {
        //     sessions: Arc::new(RwLock::new(SessionRepositoryInMemory::new())),
        // }

        Ok(Self {
            segmenter: Arc::new(Sam2Segmenter::new(model_dir)?),
        })
    }
}
