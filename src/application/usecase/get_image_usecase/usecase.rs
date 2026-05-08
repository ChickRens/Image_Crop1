use crate::application::errors::application_errors::ApplicationErrors;
use crate::application::errors::repository_errors::RepositoryErrors;
use crate::application::errors::validation::session_errors::SessionErrors;
use crate::application::errors::validation_errors::ValidationErrors;
use crate::application::repository::image_repository::ImageRepository;
use crate::application::usecase::get_image_usecase::get_image_input::GetImageInput;
use crate::application::usecase::get_image_usecase::get_image_output::GetImageOutput;
use crate::domain::repository::image_meta_repository::ImageRepository;
use crate::domain::repository::session_repository::SessionRepository;

pub struct GetImageUseCase<SR ,IR>
where
    SR: SessionRepository,
    IR: ImageRepository
{
    session_repo: SR,
    image_repo: IR,
}

impl<SR, IR> GetImageUseCase<SR, IR>
where
    SR: SessionRepository,
    IR: ImageRepository,
{
    pub fn new(
        session_repository: SR,
        image_repository: IR,
    ) -> Self {
        Self {
            session_repo: session_repository,
            image_repo: image_repository,
        }
    }

    pub fn execute(&mut self, input: GetImageInput) -> Result<GetImageOutput, ApplicationErrors> {
        let (session_id, image_id) = input.into_session_id_and_image_id();

        let session=self.session_repo.get(&session_id)
                              .ok_or(ApplicationErrors::RepositoryError(RepositoryErrors::SessionNotFound))?;
        
        let is_valid_image_id = session.has_image_id(&image_id);
        if !is_valid_image_id {
            return Err(ApplicationErrors::ValidationError(ValidationErrors::Session(SessionErrors::ImageNotOwned)))
        }
        
        let image = self.image_repo.get(&image_id)
                            .ok_or(ApplicationErrors::RepositoryError(RepositoryErrors::ImageNotFound))?;

        Ok(GetImageOutput::new(image.into_image()))
    }}