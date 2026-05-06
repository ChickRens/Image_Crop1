use std::{cell::RefCell, rc::Rc};

use crate::domain::entity::image_meta::ImageMeta;
use crate::domain::value_object::image_id::ImageId;

pub trait ImageMetaRepository {
    fn save(&mut self, meta: ImageMeta);
    fn get(&self, image_id: &ImageId) -> Option<ImageMeta>;
}

impl<T> ImageMetaRepository for Rc<RefCell<T>>
where
    T: ImageMetaRepository,
{
    fn save(&mut self, meta: ImageMeta) {
        self.borrow_mut().save(meta);
    }

    fn get(&self, image_id: &ImageId) -> Option<ImageMeta> {
        self.borrow().get(image_id)
    }
}
