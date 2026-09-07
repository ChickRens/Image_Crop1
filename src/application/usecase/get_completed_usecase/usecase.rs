use crate::{application::usecase::get_completed_usecase::{error::GetCompletedUseCaseError, input::GetCompletedInput, output::GetCompletedOutput}, domain::repository::completed_image_repository::repository::CompletedImageRepository};

pub struct GetCompletedUseCase<CR>
where
    CR: CompletedImageRepository,
{
    completed_repo: CR,
}

impl<CR> GetCompletedUseCase<CR>
where
    CR: CompletedImageRepository,
{
    pub fn new(completed_image_repository: CR) -> Self {
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
