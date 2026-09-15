use std::{
    collections::HashMap, ops::Deref, sync::{Arc, RwLock}, time::{Duration, Instant},
};

use crate::{
    application::{
        interface::{completed_image_repository::{
            error::CompletedImageRepositoryError, repository::CompletedImageRepository,
        }, delete_expired_repository::DeleteExpiredRepository}, types::{completed_image::CompletedImage, entry::Entry},
    }, domain::value_object::image_id::image_id::ImageId,
};

#[derive(Debug)]
pub struct CompletedRepositoryInMemory {
    image: RwLock<HashMap<ImageId, Entry<CompletedImage>>>,
}

impl CompletedImageRepository for CompletedRepositoryInMemory {
    fn save(&self, completed_image: CompletedImage) {
        let mut image = self
            .image
            .write()
            .expect("CompletedImageRepository is Poisoned");
        let image_id = completed_image.image_id();

        let entry = Entry::new(completed_image);
        image.insert(image_id, entry);
    }

    fn get(&self, image_id: ImageId) -> Result<CompletedImage, CompletedImageRepositoryError> {
        let image = self
            .image
            .read()
            .expect("CompletedImageRepository is Poisoned");
        image
            .get(&image_id)
            .map(|entry| entry.deref())
            .cloned()
            .ok_or(CompletedImageRepositoryError::ImageNotFound)
    }
}

impl DeleteExpiredRepository for CompletedRepositoryInMemory {
    fn delete_expired(&self, now: Instant, ttl: Duration) {
        let mut image = self.image.write().expect("CompletedImageRepository is Poisoned");
        image.retain(|_ ,image| {
            !image.is_expired(now, ttl)
        });
    }
}

impl CompletedRepositoryInMemory {
    pub fn new() -> Self {
        Self {
            image: RwLock::new(HashMap::new()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SharedCompletedImageRepository {
    repository: Arc<CompletedRepositoryInMemory>,
}

impl CompletedImageRepository for SharedCompletedImageRepository {
    fn get(&self, image_id: ImageId) -> Result<CompletedImage, CompletedImageRepositoryError> {
        self.repository.get(image_id)
    }

    fn save(&self, completed_image: CompletedImage) {
        self.repository.save(completed_image);
    }
}

impl SharedCompletedImageRepository {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(CompletedRepositoryInMemory::new()),
        }
    }
}
