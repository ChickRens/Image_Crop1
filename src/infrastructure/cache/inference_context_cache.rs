use std::collections::HashMap;

use crate::application::interface::inference_context_cache::InferenceContextCache;
use crate::application::types::segment_id::SegmentId;
use crate::infrastructure::segmenter::sam2_data::SAM2InferenceContext;

pub struct SAM2InferenceContextCache {
    contexts: HashMap<SegmentId, SAM2InferenceContext>,
}

impl InferenceContextCache for SAM2InferenceContextCache {
    type InferenceContext = SAM2InferenceContext;

    fn save(&mut self, segment_id: SegmentId, inference_context: Self::InferenceContext) {
        self.contexts.insert(segment_id, inference_context);
    }

    fn get(&self, segment_id: &SegmentId) -> Option<&Self::InferenceContext> {
        self.contexts.get(segment_id)
    }
}
