use std::collections::HashMap;

use crate::domain::repository::image_repository::ImageRepository;
use crate::domain::entity::image::Image;
use crate::domain::value_object::image_id::ImageId;
use crate::domain::value_object::image_kind::ImageKind;

pub struct ImageRepositoryInMemory {
    images: HashMap<(ImageId, ImageKind), Image>,
}

impl ImageRepository for ImageRepositoryInMemory {
    fn save(&mut self, image: Image, kind: ImageKind) {
        let key = (*image.image_id(), kind);
        self.images.insert(key, image);
    }

    fn get(&self, image_id: &ImageId, kind: ImageKind) -> Option<Image> {
        self.images.get(&(image_id.clone(), kind)).cloned()
    }
}

impl ImageRepositoryInMemory{
    pub fn new() -> Self{
        Self { images: HashMap::new() }
    }
}
