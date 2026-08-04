use std::sync::Arc;

use crate::domain::entity::image::Image;
use crate::domain::repository::image_repository::error::ImageRepositoryError;
use crate::domain::repository::image_repository::repository::ImageRepository;
use crate::domain::value_object::image_id::image_id::ImageId;
use crate::domain::value_object::image_kind::ImageKind;
use crate::infrastructure::repository::image_repository::ImageRepositoryInMemory;

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
