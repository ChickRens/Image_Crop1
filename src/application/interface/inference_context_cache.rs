use crate::application::types::segment_id::SegmentId;

pub trait InferenceContextCache {
    type InferenceContext;

    fn save(&mut self, segment_id: SegmentId, inference_context: Self::InferenceContext);
    fn get(&self, segment_id: &SegmentId) -> Option<&Self::InferenceContext>;
}
