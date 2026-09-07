use std::sync::Arc;

use crate::{application::{interface::{completed_image_generator::CompletedImageGenerator, completed_image_repository::{error::CompletedImageRepositoryError, repository::CompletedImageRepository}}, types::completed_image::CompletedImage}, domain::{entity::image::Image, value_object::image_id::image_id::ImageId}};

pub trait CompletedService {
    fn generate_and_save(&self, image: Image);
    fn get(&self, image_id: ImageId) -> Result<CompletedImage, CompletedImageRepositoryError>;
}

pub struct CompletedServiceImpl<CG, CR>
where 
    CG: CompletedImageGenerator,
    CR: CompletedImageRepository,
{
    generator: CG,
    repository: CR,
}

impl<CG, CR> CompletedService for CompletedServiceImpl<CG, CR>
where
    CG: CompletedImageGenerator,
    CR: CompletedImageRepository,
{
    fn generate_and_save(&self, image: Image) {
        let completed_image = self.generator.generate(image);
        self.repository.save(completed_image);
    }

    fn get(&self, image_id: ImageId) -> Result<CompletedImage, CompletedImageRepositoryError> {
        self.repository.get(image_id)
    }
}

pub struct SharedCompletedService<CG, CR>
where 
    CG: CompletedImageGenerator,
    CR: CompletedImageRepository,
{
    service: Arc<CompletedServiceImpl<CG, CR>>
}

impl<CG, CR> CompletedService for SharedCompletedService<CG, CR>
where 
    CG: CompletedImageGenerator,
    CR: CompletedImageRepository,
{
    fn generate_and_save(&self, image: Image) {
        self.service.generate_and_save(image);
    }

    fn get(&self, image_id: ImageId) -> Result<CompletedImage, CompletedImageRepositoryError> {
        self.service.get(image_id)
    }
}
