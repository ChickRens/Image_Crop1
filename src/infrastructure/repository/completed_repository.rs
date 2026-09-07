use std::{collections::HashMap, sync::{Arc, RwLock}};

use crate::domain::{entity::completed_image::CompletedImage, repository::completed_image_repository::{error::CompletedImageRepositoryError, repository::CompletedImageRepository}, value_object::image_id::image_id::ImageId};

pub struct CompletedRepositoryInMemory {
    image: RwLock<HashMap<ImageId, CompletedImage>>
}

impl CompletedImageRepository for CompletedRepositoryInMemory {
    fn save(&self, completed_image: CompletedImage) {
        let mut image = self.image.write().expect("CompletedImageRepository is Poisoned");
        let image_id = completed_image.image_id();
        image.insert(image_id, completed_image);
    }

    fn get(&self, image_id: ImageId) -> Result<CompletedImage, CompletedImageRepositoryError> {
        let image = self.image.read().expect("CompletedImageRepository is Poisoned");
        image.get(&image_id).cloned().ok_or(CompletedImageRepositoryError::ImageNotFound)
    }
}

pub struct SharedCompletedImageRepository {
    repository: Arc<CompletedRepositoryInMemory>
}

impl CompletedImageRepository for SharedCompletedImageRepository {
    fn get(&self, image_id: ImageId) -> Result<CompletedImage, CompletedImageRepositoryError> {
        self.repository.get(image_id)
    }

    fn save(&self, completed_image: CompletedImage) {
        self.repository.save(completed_image);
    }
}