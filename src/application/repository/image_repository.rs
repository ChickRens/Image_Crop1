use std::{cell::RefCell, rc::Rc};

use crate::application::types::image::Image;
use crate::domain::value_object::image_id::ImageId;

pub trait ImageRepository {
    fn save(&mut self, image: Image, image_id: ImageId);
    fn get(&self, image_id: &ImageId) -> Option<Image>;
}

impl<T> ImageRepository for Rc<RefCell<T>>
where
    T: ImageRepository,
{
    fn save(&mut self, image: Image, image_id: ImageId) {
        self.borrow_mut().save(image, image_id);
    }

    fn get(&self, image_id: &ImageId) -> Option<Image> {
        self.borrow().get(image_id)
    }
}