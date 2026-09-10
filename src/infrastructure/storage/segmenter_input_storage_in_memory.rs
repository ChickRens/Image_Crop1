use std::{collections::HashMap, sync::RwLock};

use crate::{
    application::{
        interface::segmenter_input_image_storage::{
            error::SegmenterInputImageStorageError, storage::SegmenterInputImageStorage,
        },
        types::segmenter_input_image::SegmenterInputImage,
    },
    domain::value_object::image_id::image_id::ImageId,
};

#[derive(Debug)]
pub struct SegmenterInputStorageInMemory {
    images: RwLock<HashMap<ImageId, SegmenterInputImage>>,
}

impl SegmenterInputImageStorage for SegmenterInputStorageInMemory {
    fn save(&self, image: SegmenterInputImage) {
        let mut images = self
            .images
            .write()
            .expect("SegmenterInputStorage is Poisoned");
        let id = image.image_id();
        images.insert(*id, image);
    }

    fn get(
        &self,
        image_id: ImageId,
    ) -> Result<SegmenterInputImage, SegmenterInputImageStorageError> {
        let images = self
            .images
            .read()
            .expect("SegmenterInputStorage is Poisoned");
        images
            .get(&image_id)
            .cloned()
            .ok_or(SegmenterInputImageStorageError::ImageNotFound)
    }
}

impl SegmenterInputStorageInMemory {
    pub fn new() -> Self {
        Self {
            images: RwLock::new(HashMap::new()),
        }
    }
}
