use std::collections::HashMap;

use crate::application::repository::image_repository::ImageRepository;
use crate::application::types::image::Image;
use crate::domain::value_object::image_id::ImageId;

pub struct ImageRepositoryInMemory {
    images: HashMap<ImageId, Image>,
}

impl ImageRepository for ImageRepositoryInMemory {
    fn save(&mut self, image: Image, image_id: ImageId) {
        self.images.insert(image_id, image);
    }

    fn get(&self, image_id: &ImageId) -> Option<Image> {
        self.images.get(image_id).cloned()
    }
}

impl ImageRepositoryInMemory{
    pub fn new() -> Self{
        Self { images: HashMap::new() }
    }
}
