use std::collections::HashMap;

use crate::domain::entity::image_meta::ImageMeta;
use crate::domain::repository::image_meta_repository::ImageMetaRepository;
use crate::domain::value_object::image_id::ImageId;

pub struct ImageMetaRepositoryInMemory {
    images: HashMap<ImageId, ImageMeta>,
}

impl ImageMetaRepository for ImageMetaRepositoryInMemory {
    fn save(&mut self, meta: ImageMeta) {
        self.images.insert(meta.image_id().clone(), meta);
    }

    fn get(&self, image_id: &ImageId) -> Option<ImageMeta> {
        self.images.get(image_id).cloned()
    }
}

impl ImageMetaRepositoryInMemory{
    pub fn new() -> Self{
        Self { images: HashMap::new() }
    }
}