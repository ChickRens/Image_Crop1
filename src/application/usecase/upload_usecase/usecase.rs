use crate::application::errors::application_errors::ApplicationErrors;
use crate::application::interface::image_loader::ImageLoader;
use crate::application::repository::image_repository::ImageRepository;
use crate::application::types::loaded_image::LoadedImage;
use crate::application::usecase::config::MAX_HISTORY;
use crate::application::usecase::upload_usecase::upload_input::UploadInput;
use crate::application::usecase::upload_usecase::upload_output::UploadOutput;
use crate::domain::entity::image_meta::ImageMeta;
use crate::domain::entity::session::Session;
use crate::domain::repository::image_meta_repository::ImageMetaRepository;
use crate::domain::repository::session_repository::SessionRepository;
use crate::domain::value_object::image_id::ImageId;
use crate::domain::value_object::session_id::SessionId;

pub struct UploadUseCase<SR, IR, MR, LD>
where
    SR: SessionRepository,
    IR: ImageRepository,
    MR: ImageMetaRepository,
    LD: ImageLoader,
{
    session_repo: SR,
    image_repo: IR,
    meta_repo: MR,
    loader: LD,
}

impl<SR, IR, MR, LD> UploadUseCase<SR, IR, MR, LD>
where
    SR: SessionRepository,
    IR: ImageRepository,
    MR: ImageMetaRepository,
    LD: ImageLoader,
{
    pub fn new(
        session_repository: SR,
        image_repository: IR,
        image_meta_repository: MR,
        image_loader: LD,
    ) -> Self {
        Self {
            session_repo: session_repository,
            image_repo: image_repository,
            meta_repo: image_meta_repository,
            loader: image_loader,
        }
    }

    pub fn execute(&mut self, input: UploadInput) -> Result<UploadOutput, ApplicationErrors> {
        let input_image = input.into_image_data();
        let image_dto: LoadedImage = self.loader.load(input_image)?;

        let image_id: ImageId = ImageId::new();

        let (image, image_size) = image_dto.into_image_and_size();
        let max_history: usize = MAX_HISTORY;
        let image_meta: ImageMeta = ImageMeta::new(image_id, image_size, max_history);

        self.image_repo.save(image, image_meta.image_id().clone());
        self.meta_repo.save(image_meta);

        let session_id: SessionId = SessionId::new();
        let session: Session = Session::new(session_id, image_id);
        self.session_repo.save(session);

        Ok(UploadOutput::new(session_id, image_id))
    }
}
