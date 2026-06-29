use std::cell::RefCell;
use std::rc::Rc;

use crate::application::errors::segmentation_error::SegmentationErrors;
use crate::application::interface::image_segmenter::ImageSegmenter;
use crate::application::types::segmented_image::SegmentedImage;
use crate::domain::entity::image::Image;
use crate::domain::value_object::point::Point;
use crate::infrastructure::segmenter::sam2::Sam2Segmenter;
use crate::infrastructure::segmenter::sam2_data::{SAM2InferenceContext, SAM2StaticContext};
type OrtResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Clone)]
pub struct SharedSAM2Segmenter {
    segmenter: Rc<RefCell<Sam2Segmenter>>,
}

impl ImageSegmenter for SharedSAM2Segmenter {
    type StaticContext = SAM2StaticContext;
    type InferenceContext = SAM2InferenceContext;
    // fn save(&mut self, session: Session) {
    //     self.sessions.borrow_mut().save(session);
    // }
    // fn get(&self, session_id: &SessionId) -> Option<Session> {
    //     self.sessions.borrow().get(session_id)
    // }

    fn segment(
        &mut self,
        original_image: &Image,
        static_context: &Self::StaticContext,
        inference_context: &Self::InferenceContext,
        input_points: &[Point],
    ) -> Result<(Self::InferenceContext, SegmentedImage), SegmentationErrors> {
        self.segmenter.borrow_mut().segment(
            original_image,
            static_context,
            inference_context,
            input_points,
        )
    }

    fn rebuild(
        &mut self,
        original_image: &Image,
        static_context: &Self::StaticContext,
        input_points: &[Point],
    ) -> Result<(Self::InferenceContext, SegmentedImage), SegmentationErrors> {
        self.segmenter
            .borrow_mut()
            .rebuild(original_image, static_context, input_points)
    }
    fn prepare_inference_context(
        &mut self,
        image: &Image,
    ) -> Result<Self::InferenceContext, SegmentationErrors> {
        self.segmenter.borrow_mut().prepare_inference_context(image)
    }

    fn prepare_static_context(
        &mut self,
        image: &Image,
    ) -> Result<Self::StaticContext, SegmentationErrors> {
        self.segmenter.borrow_mut().prepare_static_context(image)
    }
}

impl SharedSAM2Segmenter {
    pub fn new(model_dir: &str) -> OrtResult<Self> {
        // Self {
        //     sessions: Rc::new(RefCell::new(SessionRepositoryInMemory::new())),
        // }

        Ok(Self {
            segmenter: Rc::new(RefCell::new(Sam2Segmenter::new(model_dir)?)),
        })
    }
}
