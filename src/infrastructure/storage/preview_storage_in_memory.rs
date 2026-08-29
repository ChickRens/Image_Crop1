use std::{collections::HashMap, sync::Mutex};

use crate::{application::{interface::preview_storage::{error::PreviewStorageError, storage::PreviewStorage}, types::preview_image::PreviewImage}, domain::value_object::image_id::image_id::ImageId};

#[derive(Debug)]
pub struct PreviewStorageInMemory {
    images: Mutex<HashMap<ImageId, PreviewImage>>
}

impl PreviewStorage for PreviewStorageInMemory {
    fn save(&self, image: PreviewImage) {
        let mut images = self.images.lock().expect("PreviewStorage is Poisoned");
        let image_id = image.image_id();
        images.insert(image_id, image);
    }

    fn get(&self, image_id: ImageId) -> Result<PreviewImage, PreviewStorageError> {
        let images = self.images.lock().expect("PreviewStorage is Poisoned");
        images.get(&image_id).cloned().ok_or(PreviewStorageError::ImageNotFound)
    }
}

impl PreviewStorageInMemory {
    pub fn new() -> Self {
        Self { images: Mutex::new(HashMap::new()) }
    }
}

