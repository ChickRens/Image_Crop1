use crate::application::errors::application_errors::ApplicationErrors;
use crate::application::interface::editing_session_repository::EditingSessionRepository;
use crate::application::interface::image_loader::ImageLoader;
use crate::application::interface::image_segmenter::ImageSegmenter;
use crate::application::types::editing_session::CommonEditingSession;
use crate::application::types::loaded_image::LoadedImage;
use crate::application::types::point_history::PointHistory;
use crate::application::usecase::config::MAX_HISTORY;
use crate::application::usecase::upload_usecase::upload_input::UploadInput;
use crate::application::usecase::upload_usecase::upload_output::UploadOutput;
use crate::domain::entity::image::Image;
use crate::domain::entity::session::Session;
use crate::domain::repository::image_repository::ImageRepository;
use crate::domain::repository::session_repository::SessionRepository;
use crate::domain::value_object::image_id::ImageId;
use crate::domain::value_object::image_kind::ImageKind;
use crate::domain::value_object::session_id::SessionId;

pub struct UploadUseCase<SR, IR, LD, IS, ESR>
where
    SR: SessionRepository,
    IR: ImageRepository,
    LD: ImageLoader,
    IS: ImageSegmenter,
    ESR: EditingSessionRepository<
            StaticContext = IS::StaticContext,
            InferenceContext = IS::InferenceContext,
        >,
{
    session_repo: SR,
    image_repo: IR,
    loader: LD,
    segmenter: IS,
    editing_session_repo: ESR,
}

impl<SR, IR, LD, IS, ESR> UploadUseCase<SR, IR, LD, IS, ESR>
where
    SR: SessionRepository,
    IR: ImageRepository,
    LD: ImageLoader,
    IS: ImageSegmenter,
    ESR: EditingSessionRepository<
            StaticContext = IS::StaticContext,
            InferenceContext = IS::InferenceContext,
        >,
{
    pub fn new(
        session_repository: SR,
        image_repository: IR,
        image_loader: LD,
        image_segmenter: IS,
        editing_session_repository: ESR,
    ) -> Self {
        Self {
            session_repo: session_repository,
            image_repo: image_repository,
            loader: image_loader,
            segmenter: image_segmenter,
            editing_session_repo: editing_session_repository,
        }
    }

    pub fn execute(&mut self, input: UploadInput) -> Result<UploadOutput, ApplicationErrors> {
        let input_image = input.into_image_data();
        let image_dto: LoadedImage = self.loader.load(input_image)?;

        let image_id: ImageId = ImageId::new();

        let image: Image = image_dto.into_image();

        let session_id: SessionId = SessionId::new();
        let session: Session = Session::new(session_id, image_id);
        self.session_repo.save(session);

        let inference_context = self.segmenter.prepare_inference_context(&image)?;
        let static_context = self.segmenter.prepare_static_context(&image)?;
        let history = PointHistory::new(MAX_HISTORY);
        let editing_session = CommonEditingSession::new(history, static_context, inference_context);

        self.image_repo.save(image, ImageKind::Original);
        self.editing_session_repo.save(&session_id, editing_session);

        let output = UploadOutput::new(session_id, image_id);

        Ok(output)
    }
}
