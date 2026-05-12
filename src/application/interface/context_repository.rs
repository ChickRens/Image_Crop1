use std::task::Context;

use crate::application::types::crop_id::SegmentId;

pub trait ContextRepository {
    type Context;

    fn save(&mut self, context: Self::Context);
    fn get(&self, segment_id:SegmentId) -> Context;
}