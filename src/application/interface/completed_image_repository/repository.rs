use crate::{application::{interface::completed_image_repository::error::CompletedImageRepositoryError, types::completed_image::CompletedImage}, domain::value_object::image_id::image_id::ImageId};

pub trait CompletedImageRepository {
    fn save(&self, completed_image: CompletedImage);
    fn get(&self, image_id: ImageId) -> Result<CompletedImage, CompletedImageRepositoryError>;
}