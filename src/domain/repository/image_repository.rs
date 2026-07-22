use crate::domain::entity::image::Image;
use crate::domain::value_object::image_id::ImageId;
use crate::domain::value_object::image_kind::ImageKind;

pub trait ImageRepository {
    fn save(&self, image: Image, kind: ImageKind);
    fn get(&self, image_id: &ImageId, kind: ImageKind) -> Option<Image>;
}
