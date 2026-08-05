use crate::domain::entity::image::image::Image;
use crate::domain::repository::image_repository::error::ImageRepositoryError;
use crate::domain::value_object::image_id::image_id::ImageId;

pub trait ImageRepository {
    fn save(&self, image: Image);
    fn get(&self, image_id: &ImageId) -> Result<Image, ImageRepositoryError>;
}
