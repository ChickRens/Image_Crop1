use crate::application::{service::completed_service::CompletedService, usecase::get_completed_usecase::{error::GetCompletedUseCaseError, input::GetCompletedInput, output::GetCompletedOutput}};

pub struct GetCompletedUseCase<CS>
where
    CS: CompletedService,
{
    completed_repo: CS,
}

impl<CS> GetCompletedUseCase<CS>
where
    CS: CompletedService,
{
    pub fn new(completed_image_repository: CS) -> Self {
        Self { completed_repo: completed_image_repository }
    }

    pub fn execute(&self, input: GetCompletedInput) -> Result<GetCompletedOutput, GetCompletedUseCaseError> {
        let image_id = input.image_id();

        let completed = self.completed_repo.get(image_id)?;

        let image = completed.into_image();
        let (data, _, size) = image.into_data();

        let output = GetCompletedOutput::new(data.into_image(), size.clone());

        Ok(output)
    }
}
