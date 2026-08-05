use std::sync::Arc;

use crate::{domain::{entity::image::image::Image, repository::image_repository::{error::ImageRepositoryError, repository::ImageRepository}, value_object::{image_id::image_id::ImageId}}, infrastructure::repository::image_repository::ImageRepositoryInMemory};

#[derive(Clone)]
pub struct SharedImageRepository {
    images: Arc<ImageRepositoryInMemory>,
}

impl SharedImageRepository {
    pub fn new() -> Self {
        Self {
            images: Arc::new(ImageRepositoryInMemory::new()),
        }
    }
}

impl ImageRepository for SharedImageRepository {
    fn get(&self, image_id: &ImageId) -> Result<Image, ImageRepositoryError> {
        self.images.get(image_id)
    }

    fn save(&self, image: Image) {
        self.images.save(image);
    }
}
