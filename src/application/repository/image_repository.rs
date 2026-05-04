use crate::application::types::image::Image;
use crate::domain::value_object::image_id::ImageId;

pub trait ImageRepository {
    fn save(&mut self, image: Image, image_id: ImageId);
    fn get(&self, image_id: &ImageId) -> Option<Image>;
}