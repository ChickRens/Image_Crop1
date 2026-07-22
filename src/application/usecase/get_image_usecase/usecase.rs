use crate::application::errors::application_errors::ApplicationErrors;
use crate::application::errors::repository_errors::RepositoryErrors;
use crate::application::errors::validation::session_errors::SessionErrors;
use crate::application::errors::validation_errors::ValidationErrors;
use crate::application::usecase::get_image_usecase::get_image_input::GetImageInput;
use crate::application::usecase::get_image_usecase::get_image_output::GetImageOutput;
use crate::domain::entity::image::Image;
use crate::domain::entity::session::Session;
use crate::domain::repository::image_repository::ImageRepository;
use crate::domain::repository::session_repository::SessionRepository;

pub struct GetImageUseCase<SR, IR>
where
    SR: SessionRepository,
    IR: ImageRepository,
{
    session_repo: SR,
    image_repo: IR,
}

impl<SR, IR> GetImageUseCase<SR, IR>
where
    SR: SessionRepository,
    IR: ImageRepository,
{
    pub fn new(session_repository: SR, image_repository: IR) -> Self {
        Self {
            session_repo: session_repository,
            image_repo: image_repository,
        }
    }

    pub fn execute(&self, input: GetImageInput) -> Result<GetImageOutput, ApplicationErrors> {
        let (session_id, image_id, image_kind) = input.into_session_id_image_id_image_kind();

        let session: Session =
            self.session_repo
                .get(&session_id)
                .ok_or(ApplicationErrors::RepositoryError(
                    RepositoryErrors::SessionNotFound,
                ))?;

        let is_valid_image_id: bool = session.has_image_id(&image_id);
        if !is_valid_image_id {
            return Err(ApplicationErrors::ValidationError(
                ValidationErrors::Session(SessionErrors::ImageNotOwned),
            ));
        }

        let image: Image = self.image_repo.get(&image_id, image_kind).ok_or(
            ApplicationErrors::RepositoryError(RepositoryErrors::ImageNotFound),
        )?;

        let (data, _id, _size) = image.into_data();
        let output: GetImageOutput = GetImageOutput::new(data.into_image());

        Ok(output)
    }
}
