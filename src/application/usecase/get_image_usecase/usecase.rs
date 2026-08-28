use crate::application::{
    interface::preview_storage::storage::PreviewStorage, usecase::get_image_usecase::{
        error::GetImageUseCaseError, get_image_input::GetImageInput,
        get_image_output::GetImageOutput,
    },
};

pub struct GetImageUseCase<PS>
where
    PS: PreviewStorage
{
    storage: PS,
}

impl<PS> GetImageUseCase<PS>
where
    PS: PreviewStorage,
{
    pub fn new(preview_storage: PS) -> Self {
        Self { storage: preview_storage }
    }

    pub fn execute(&self, input: GetImageInput) -> Result<GetImageOutput, GetImageUseCaseError> {
        let image_id = input.image_id();

        let preview = self.storage.get(image_id)?;

        let (data, _, size) = preview.into_data();

        let output = GetImageOutput::new(data.into_image(), size.clone());

        Ok(output)
    }
}
