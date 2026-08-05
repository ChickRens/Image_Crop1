use crate::domain::entity::image::image::Image;
use crate::domain::repository::image_repository::error::ImageRepositoryError;
use crate::domain::value_object::image_id::image_id::ImageId;
use crate::domain::value_object::image_kind::ImageKind;

pub trait ImageRepository {
    fn save(&self, image: Image, kind: ImageKind);
    fn get(&self, image_id: &ImageId, kind: ImageKind) -> Result<Image, ImageRepositoryError>;
}
