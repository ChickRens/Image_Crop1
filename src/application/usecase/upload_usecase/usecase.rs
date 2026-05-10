use crate::application::errors::application_errors::ApplicationErrors;
use crate::application::interface::image_loader::ImageLoader;
use crate::application::types::loaded_image::LoadedImage;
use crate::application::usecase::upload_usecase::upload_input::UploadInput;
use crate::domain::entity::image::Image;
use crate::domain::entity::session::Session;
use crate::domain::repository::image_repository::ImageRepository;
use crate::domain::repository::session_repository::SessionRepository;
use crate::domain::value_object::image_id::ImageId;
use crate::domain::value_object::session_id::SessionId;

pub struct UploadUseCase<SR, IR, LD>
where
    SR: SessionRepository,
    IR: ImageRepository,
    LD: ImageLoader,
{
    session_repo: SR,
    image_repo: IR,
    loader: LD,
}

impl<SR, IR, LD> UploadUseCase<SR, IR, LD>
where
    SR: SessionRepository,
    IR: ImageRepository,
    LD: ImageLoader,
{
    pub fn new(
        session_repository: SR,
        image_repository: IR,
        image_loader: LD,
    ) -> Self {
        Self {
            session_repo: session_repository,
            image_repo: image_repository,
            loader: image_loader,
        }
    }

    pub fn execute(&mut self, input: UploadInput) -> Result<(), ApplicationErrors> {
        let input_image = input.into_image_data();
        let image_dto: LoadedImage = self.loader.load(input_image)?;

        let image_id: ImageId = ImageId::new();

        let image: Image = image_dto.into_image();

        self.image_repo.save(image);

        let session_id: SessionId = SessionId::new();
        let session: Session = Session::new(session_id, image_id);
        self.session_repo.save(session);

        Ok(())
    }
}
