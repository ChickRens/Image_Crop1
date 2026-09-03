use std::{sync::Arc, time::Instant};

use crate::{application::{interface::{preview_image_generator::PreviewImageGenerator, preview_storage::{error::PreviewStorageError, storage::PreviewStorage}}, types::preview_image::PreviewImage}, domain::{entity::image::Image, value_object::image_id::image_id::ImageId}};

pub trait PreviewService {
    fn generate_and_save(&self, image: &Image) -> f64;
    fn get(&self, image_id: ImageId) -> Result<PreviewImage, PreviewStorageError>;
}

pub struct PreviewServiceImpl<PG, PS>
where
    PG: PreviewImageGenerator,
    PS: PreviewStorage,
{
    generator: PG,
    storage: PS,
}

impl<PG, PS> PreviewService for PreviewServiceImpl<PG, PS>
where 
    PG: PreviewImageGenerator,
    PS: PreviewStorage,
{
    fn generate_and_save(&self, image: &Image) -> f64 {
        let start = Instant::now();
        let (preview, scale) = self.generator.generate(image);
        let end = start.elapsed();
        println!("Preview Generate: {:?}", end);

        self.storage.save(preview);
        scale
    }

    fn get(&self, image_id: ImageId) -> Result<PreviewImage, PreviewStorageError> {
        self.storage.get(image_id)
    }
}

impl<PG, PS> PreviewServiceImpl<PG, PS>
where 
    PG: PreviewImageGenerator,
    PS: PreviewStorage,
{
    pub fn new(preview_generator: PG, preview_storage: PS) -> Self {
        Self { generator: preview_generator, storage: preview_storage }
    }
}

#[derive(Clone)]
pub struct SharedPreviewService<PG, PS> 
where
    PG: PreviewImageGenerator,
    PS: PreviewStorage,
{
    service: Arc<PreviewServiceImpl<PG, PS>>
}

impl<PG, PS> PreviewService for SharedPreviewService<PG, PS> 
where
    PG: PreviewImageGenerator,
    PS: PreviewStorage,
{
    fn generate_and_save(&self, image: &Image) -> f64 {
        self.service.generate_and_save(image)
    }

    fn get(&self, image_id: ImageId) -> Result<PreviewImage, PreviewStorageError> {
        self.service.get(image_id)
    }
}

impl<PG, PS> SharedPreviewService<PG, PS> 
where
    PG: PreviewImageGenerator,
    PS: PreviewStorage,
{
    pub fn new(preview_generator: PG, preview_storage: PS) -> Self{
        Self { service: Arc::new(PreviewServiceImpl::new(preview_generator, preview_storage)) }
    }
}