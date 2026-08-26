use std::sync::Arc;

use crate::{
    domain::{
        entity::original_image::OriginalImage, repository::original_image_repository::{
            error::OriginalImageRepositoryError, repository::OriginalImageRepository,
        }, value_object::image_id::image_id::ImageId,
    }, infrastructure::repository::image_repository::OriginalImageRepositoryInMemory,
};

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
    fn get(&self, image_id: &ImageId) -> Result<OriginalImage, OriginalImageRepositoryError> {
        self.images.get(image_id)
    }

    fn save(&self, image: OriginalImage) {
        self.images.save(image);
    }
}
