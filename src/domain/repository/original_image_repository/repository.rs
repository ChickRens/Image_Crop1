use crate::domain::entity::image::Image;
use crate::domain::repository::original_image_repository::error::OriginalImageRepositoryError;
use crate::domain::value_object::image_id::image_id::ImageId;

pub trait OriginalImageRepository {
    fn save(&self, image: Image);
    fn get(&self, image_id: &ImageId) -> Result<Image, OriginalImageRepositoryError>;
}
