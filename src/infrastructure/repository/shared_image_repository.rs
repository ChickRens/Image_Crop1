use std::sync::Arc;

use crate::{domain::{entity::image::image::Image, repository::original_image_repository::{error::OriginalImageRepositoryError, repository::OriginalImageRepository}, value_object::{image_id::image_id::ImageId}}, infrastructure::repository::image_repository::OriginalImageRepositoryInMemory};

#[derive(Clone)]
pub struct SharedOriginalImageRepository {
    images: Arc<OriginalImageRepositoryInMemory>,
}

impl SharedOriginalImageRepository {
    pub fn new() -> Self {
        Self {
            images: Arc::new(OriginalImageRepositoryInMemory::new()),
        }
    }
}

impl OriginalImageRepository for SharedOriginalImageRepository {
    fn get(&self, image_id: &ImageId) -> Result<Image, OriginalImageRepositoryError> {
        self.images.get(image_id)
    }

    fn save(&self, image: Image) {
        self.images.save(image);
    }
}
