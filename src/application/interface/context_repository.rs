use crate::application::types::segment_id::SegmentId;

pub trait ContextRepository {
    type Context;

    fn save(&mut self, context: Self::Context);
    fn get(&self, segment_id: SegmentId) -> Self::Context;
}
