use std::cell::RefCell;
use std::rc::Rc;

use crate::domain::entity::image::Image;
use crate::domain::repository::image_repository::ImageRepository;
use crate::domain::value_object::image_id::ImageId;
use crate::domain::value_object::image_kind::ImageKind;
use crate::infrastructure::repository::image_repository::ImageRepositoryInMemory;

#[derive(Clone)]
pub struct SharedImageRepository {
    images: Rc<RefCell<ImageRepositoryInMemory>>
}

impl SharedImageRepository {
    pub fn new() -> Self {
        Self { images: Rc::new(RefCell::new(ImageRepositoryInMemory::new())) }
    }
}

impl ImageRepository for SharedImageRepository {
    fn get(&self, image_id: &ImageId, kind: ImageKind) -> Option<Image> {
        self.images.borrow().get(image_id, kind)
    }

    fn save(&mut self, image: Image, kind: ImageKind) {
        self.images.borrow_mut().save(image, kind);
    }
}
