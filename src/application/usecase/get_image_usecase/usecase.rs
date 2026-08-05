use crate::{application::usecase::get_image_usecase::{error::GetImageUseCaseError, get_image_input::GetImageInput, get_image_output::GetImageOutput}, domain::repository::image_repository::repository::ImageRepository};

pub struct GetImageUseCase<IR>
where
    IR: ImageRepository,
{
    image_repo: IR,
}

impl<IR> GetImageUseCase<IR>
where
    IR: ImageRepository,
{
    pub fn new(image_repository: IR) -> Self {
        Self {
            image_repo: image_repository,
        }
    }

    pub fn execute(&self, input: GetImageInput) -> Result<GetImageOutput, GetImageUseCaseError> {
        let image_id = input.image_id();

        let image = self.image_repo.get(&image_id)?;

        let (data, _id, _size) = image.into_data();
        let output= GetImageOutput::new(data.into_image());

        Ok(output)
    }
}
