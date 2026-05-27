use crate::application::types::segment_id::SegmentId;

pub trait ContextCache {
    type SegmentationContext;

    fn save(&mut self, context: Self::SegmentationContext);
    fn get(&self, segment_id: &SegmentId) -> Option<&Self::SegmentationContext>;
}
