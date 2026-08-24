use crate::domain::entity::original_image::OriginalImage;
use crate::domain::repository::original_image_repository::error::OriginalImageRepositoryError;
use crate::domain::value_object::image_id::image_id::ImageId;

pub trait OriginalImageRepository {
    fn save(&self, image: OriginalImage);
    fn get(&self, image_id: &ImageId) -> Result<OriginalImage, OriginalImageRepositoryError>;
}
