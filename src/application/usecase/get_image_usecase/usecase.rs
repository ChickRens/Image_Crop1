use crate::application::{
    interface::{preview_image_generator::PreviewImageGenerator, preview_storage::storage::PreviewStorage}, service::preview_service::PreviewService, usecase::get_image_usecase::{
        error::GetImageUseCaseError, get_image_input::GetImageInput,
        get_image_output::GetImageOutput,
    },
};

pub struct GetImageUseCase<PG, PS>
where
    PG: PreviewImageGenerator,
    PS: PreviewStorage,
{
    preview_service: PreviewService<PG, PS>
}

impl<PG, PS> GetImageUseCase<PG, PS>
where
    PG: PreviewImageGenerator,
    PS: PreviewStorage,
{
    pub fn new(preview_service: PreviewService<PG, PS>) -> Self {
        Self { preview_service }
    }

    pub fn execute(&self, input: GetImageInput) -> Result<GetImageOutput, GetImageUseCaseError> {
        let image_id = input.image_id();

        let preview = self.preview_service.get(image_id)?;

        let (data, _, size) = preview.into_data();

        let output = GetImageOutput::new(data.into_image(), size.clone());

        Ok(output)
    }
}
