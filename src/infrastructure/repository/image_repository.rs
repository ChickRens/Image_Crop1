use std::{collections::HashMap, sync::Mutex};

use crate::domain::{
    entity::image::Image,
    repository::original_image_repository::{
        error::OriginalImageRepositoryError, repository::OriginalImageRepository,
    },
    value_object::image_id::image_id::ImageId,
};

pub struct OriginalImageRepositoryInMemory {
    images: Mutex<HashMap<ImageId, Image>>,
}

impl OriginalImageRepository for OriginalImageRepositoryInMemory {
    fn save(&self, image: Image) {
        let key = *image.image_id();
        let mut images = self
            .images
            .lock()
            .expect("ImageRepositoryInMemory is Poisoned");
        images.insert(key, image);
    }

    fn get(&self, image_id: &ImageId) -> Result<Image, OriginalImageRepositoryError> {
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
