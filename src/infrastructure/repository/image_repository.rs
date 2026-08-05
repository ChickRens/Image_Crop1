use std::{collections::HashMap, sync::Mutex};

use crate::domain::{entity::image::image::Image, repository::image_repository::{error::ImageRepositoryError, repository::ImageRepository}, value_object::{image_id::image_id::ImageId, image_kind::ImageKind}};

pub struct ImageRepositoryInMemory {
    images: Mutex<HashMap<(ImageId, ImageKind), Image>>,
}

impl ImageRepository for ImageRepositoryInMemory {
    fn save(&self, image: Image, kind: ImageKind) {
        let key = (*image.image_id(), kind);
        let mut images = self
            .images
            .lock()
            .expect("ImageRepositoryInMemory is Poisoned");
        images.insert(key, image);
    }

    fn get(&self, image_id: &ImageId, kind: ImageKind) -> Result<Image, ImageRepositoryError> {
        let images = self
            .images
            .lock()
            .expect("ImageRepositoryInMemory is Poisoned");
        images.get(&(image_id.clone(), kind)).cloned().ok_or(ImageRepositoryError::ImageNotFound)
    }
}

impl ImageRepositoryInMemory {
    pub fn new() -> Self {
        Self {
            images: Mutex::new(HashMap::new()),
        }
    }
}
