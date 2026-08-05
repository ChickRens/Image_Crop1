use std::sync::Arc;

use crate::{domain::{entity::image::image::Image, repository::image_repository::{error::ImageRepositoryError, repository::ImageRepository}, value_object::{image_id::image_id::ImageId, image_kind::ImageKind}}, infrastructure::repository::image_repository::ImageRepositoryInMemory};

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
    fn get(&self, image_id: &ImageId, kind: ImageKind) -> Result<Image, ImageRepositoryError> {
        self.images.get(image_id, kind)
    }

    fn save(&self, image: Image, kind: ImageKind) {
        self.images.save(image, kind);
    }
}
