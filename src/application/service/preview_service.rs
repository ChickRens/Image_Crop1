use crate::{application::{interface::{preview_image_generator::PreviewImageGenerator, preview_storage::{error::PreviewStorageError, storage::PreviewStorage}}, types::preview_image::PreviewImage}, domain::{entity::image::Image, value_object::image_id::image_id::ImageId}};

pub struct PreviewService<PG, PS>
where
    PG: PreviewImageGenerator,
    PS: PreviewStorage,
{
    generator: PG,
    storage: PS,
}

impl<PG, PS> PreviewService<PG, PS>
where 
    PG: PreviewImageGenerator,
    PS: PreviewStorage,
{
    pub fn new(preview_generator: PG, preview_storage: PS) -> Self {
        Self { generator: preview_generator, storage: preview_storage }
    }

    pub fn generate_and_save(&self, image: &Image) {
        let (preview, _) = self.generator.generate(image);
        self.storage.save(preview);
    }

    pub fn get(&self, image_id: ImageId) -> Result<PreviewImage, PreviewStorageError> {
        self.storage.get(image_id)
    }
}