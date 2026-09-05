use std::{collections::HashMap, sync::Mutex};

use crate::domain::{
    entity::original_image::OriginalImage,
    repository::original_image_repository::{
        error::OriginalImageRepositoryError, repository::OriginalImageRepository,
    },
    value_object::image_id::image_id::ImageId,
};

pub struct OriginalImageRepositoryInMemory {
    images: Mutex<HashMap<ImageId, OriginalImage>>,
}

impl OriginalImageRepository for OriginalImageRepositoryInMemory {
    fn save(&self, image: OriginalImage) {
        let key = image.image_id();
        let mut images = self
            .images
            .lock()
            .expect("ImageRepositoryInMemory is Poisoned");
        images.insert(key, image);
    }

    fn get(&self, image_id: &ImageId) -> Result<OriginalImage, OriginalImageRepositoryError> {
        let images = self
            .images
            .lock()
            .expect("ImageRepositoryInMemory is Poisoned");
        images
            .get(&image_id.clone())
            .cloned()
            .ok_or(OriginalImageRepositoryError::ImageNotFound)
    }
}

impl OriginalImageRepositoryInMemory {
    pub fn new() -> Self {
        Self {
            images: Mutex::new(HashMap::new()),
        }
    }
}
