use std::sync::Arc;

use crate::{application::{interface::segmenter_input_image_storage::{error::SegmenterInputImageStorageError, storage::SegmenterInputImageStorage}, types::segmenter_input_image::SegmenterInputImage}, domain::value_object::image_id::image_id::ImageId, infrastructure::storage::segmenter_input_storage_in_memory::SegmenterInputStorageInMemory};

#[derive(Debug, Clone)]
pub struct SharedSegmenterInputStorage {
    storage: Arc<SegmenterInputStorageInMemory>
}

impl SegmenterInputImageStorage for SharedSegmenterInputStorage {
    fn save(&self, image: SegmenterInputImage) {
        self.storage.save(image);
    }

    fn get(&self, image_id: ImageId) -> Result<SegmenterInputImage, SegmenterInputImageStorageError> {
        self.storage.get(image_id)
    }
}

impl SharedSegmenterInputStorage {
    pub fn new() -> Self {
        Self { storage: Arc::new(SegmenterInputStorageInMemory::new()) }
    }
}