use std::sync::Arc;

use crate::{application::{interface::preview_storage::{error::PreviewStorageError, storage::PreviewStorage}, types::preview_image::PreviewImage}, domain::value_object::image_id::image_id::ImageId, infrastructure::storage::preview_storage_in_memory::PreviewStorageInMemory};

pub struct SharedPreviewStorage {
    storage: Arc<PreviewStorageInMemory>
}

impl PreviewStorage for SharedPreviewStorage {
    fn save(&self, image: PreviewImage) {
        self.storage.save(image);
    }

    fn get(&self, image_id: ImageId) -> Result<PreviewImage, PreviewStorageError> {
        self.storage.get(image_id)
    }
}

impl SharedPreviewStorage {
    pub fn new() -> Self {
        Self { storage: Arc::new(PreviewStorageInMemory::new()) }
    }
}