use crate::application::{
    service::preview_service::PreviewService, usecase::get_image_usecase::{
        error::GetImageUseCaseError, get_image_input::GetImageInput,
        get_image_output::GetImageOutput,
    },
};

pub struct GetImageUseCase<PS>
where
    PS: PreviewService
{
    preview_service: PS
}

impl<PS> GetImageUseCase<PS>
where
    PS: PreviewService
{
    pub fn new(preview_service: PS) -> Self {
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
