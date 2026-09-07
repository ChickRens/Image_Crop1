use crate::{application::{interface::completed_image_repository::error::CompletedImageRepositoryError, types::completed_image::CompletedImage}, domain::{entity::image::Image, value_object::image_id::image_id::ImageId}};

pub trait CompletedService {
    fn generate_and_save(&self, image: Image);
    fn get(&self, image_id: ImageId) -> Result<CompletedImage, CompletedImageRepositoryError>;
}