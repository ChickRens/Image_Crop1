use crate::domain::entity::image::Image;
use crate::domain::value_object::image_id::ImageId;

pub trait ImageRepository {
    fn save(&mut self, meta: Image);
    fn get(&self, image_id: &ImageId) -> Option<Image>;
}
