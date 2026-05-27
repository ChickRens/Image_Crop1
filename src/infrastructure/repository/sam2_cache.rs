use std::collections::HashMap;
use crate::application::interface::context_cache::ContextCache;
use crate::application::types::segment_id::SegmentId;
use crate::infrastructure::segmenter::sam2_data::{SAM2InferenceContext, SAM2StaticContext};

pub struct SAM2CacheInMemory {
    static_context: SAM2StaticContext,
    runtime_contexts: HashMap<SegmentId, SAM2Context>,
}

#[derive(Debug)]
pub struct SAM2Context {
    runtime_context: SAM2InferenceContext,
    segment_id: SegmentId,
}

impl SAM2Context {
    fn new(runtime_context: SAM2InferenceContext, segment_id: SegmentId) -> Self {
        Self {
            runtime_context,
            segment_id,
        }
    }

    fn segment_id(&self) -> &SegmentId {
        &self.segment_id
    }
}

impl ContextCache for SAM2CacheInMemory {
    type SegmentationContext = SAM2Context;

    fn save(&mut self, context: Self::SegmentationContext) {
        self.runtime_contexts.insert(*context.segment_id(), context);
    }
    fn get(&self, segment_id: &SegmentId) -> Option<&Self::SegmentationContext> {
        self.runtime_contexts.get(segment_id)
    }
}
