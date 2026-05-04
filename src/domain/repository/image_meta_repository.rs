use crate::domain::entity::image_meta::ImageMeta;
use crate::domain::value_object::image_id::ImageId;

pub trait ImageMetaRepository {
    fn save(&mut self, meta: ImageMeta);
    fn get(&self, image_id: &ImageId) -> Option<ImageMeta>;
}
