use crate::application::{
    interface::rendered_image_cache::cache::RenderedImageCache,
    usecase::get_image_usecase::{
        error::GetImageUseCaseError, get_image_input::GetImageInput,
        get_image_output::GetImageOutput,
    },
};

pub struct GetImageUseCase<IC>
where
    IC: RenderedImageCache,
{
    image_cache: IC,
}

impl<IC> GetImageUseCase<IC>
where
    IC: RenderedImageCache,
{
    pub fn new(image_cache: IC) -> Self {
        Self { image_cache }
    }

    pub fn execute(&self, input: GetImageInput) -> Result<GetImageOutput, GetImageUseCaseError> {
        let image_id = input.image_id();

        let image = self.image_cache.take(image_id)?;

        let (data, _id, size) = image.into_data();
        let output = GetImageOutput::new(data.into_image(), size);

        Ok(output)
    }
}
