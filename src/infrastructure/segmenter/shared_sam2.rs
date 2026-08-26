use std::sync::Arc;

use crate::{
    application::{
        interface::image_segmenter::{error::{SegmenterLoadingError, SegmenterModelError, SegmenterRuntimeError}, segmenter::ImageSegmenter}, types::segmented_image::SegmentedImage,
    }, domain::{entity::original_image::OriginalImage, value_object::point::Point}, infrastructure::segmenter::{
        sam2::Sam2Segmenter,
        sam2_data::{SAM2InferenceContext, SAM2StaticContext},
    },
};

#[derive(Clone)]
pub struct SharedSAM2Segmenter {
    segmenter: Arc<Sam2Segmenter>,
}

impl ImageSegmenter for SharedSAM2Segmenter {
    type StaticContext = SAM2StaticContext;
    type InferenceContext = SAM2InferenceContext;

    fn segment(
        &self,
        original_image: &OriginalImage,
        static_context: &Self::StaticContext,
        inference_context: &Self::InferenceContext,
        input_points: &[Point],
    ) -> Result<(Self::InferenceContext, SegmentedImage), SegmenterRuntimeError> {
        self.segmenter.segment(
            original_image,
            static_context,
            inference_context,
            input_points,
        )
    }

    fn prepare_inference_context(
        &self,
        image: &OriginalImage,
    ) -> Result<Self::InferenceContext, SegmenterLoadingError> {
        self.segmenter.prepare_inference_context(image)
    }

    fn prepare_static_context(&self, image: &OriginalImage) -> Result<Self::StaticContext, SegmenterLoadingError> {
        self.segmenter.prepare_static_context(image)
    }
}

impl SharedSAM2Segmenter {
    pub fn new(model_dir: &str) -> Result<Self, SegmenterModelError> {
        // Self {
        //     sessions: Arc::new(RwLock::new(SessionRepositoryInMemory::new())),
        // }

        Ok(Self {
            segmenter: Arc::new(Sam2Segmenter::new(model_dir)?),
        })
    }
}
