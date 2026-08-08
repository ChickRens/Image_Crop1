use crate::{application::{interface::{editing_session_repository::repository::EditingSessionRepository, image_loader::loader::ImageLoader, image_segmenter::segmenter::ImageSegmenter}, types::{editing_session::session::{CommonEditingSession, EditingSession}, inference_context_history::InferenceContextHistory, point_history::PointHistory}, usecase::{config::MAX_HISTORY, upload_usecase::{error::UploadUseCaseError, upload_input::UploadInput, upload_output::UploadOutput}}}, domain::{entity::session::session::Session, repository::{original_image_repository::repository::OriginalImageRepository, session_repository::repository::SessionRepository}, value_object::{image_id::image_id::ImageId, session_id::session_id::SessionId}}};

pub struct UploadUseCase<SR, IR, LD, IS, ESR>
where
    SR: SessionRepository,
    IR: OriginalImageRepository,
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
    IR: OriginalImageRepository,
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

    pub fn execute(&self, input: UploadInput) -> Result<UploadOutput, UploadUseCaseError> {
        let input_image = input.into_image_data();
        let loaded_image = self.loader.load(input_image)?;

        let image_id= ImageId::new();

        let image = loaded_image.into_image();

        let session_id = SessionId::new();
        let session = Session::new(session_id, image_id);
        self.session_repo.save(session);

        let inference_context = self.segmenter.prepare_inference_context(&image)?;
        let static_context = self.segmenter.prepare_static_context(&image)?;

        let point_history = PointHistory::new(MAX_HISTORY);
        let context_history = InferenceContextHistory::new(MAX_HISTORY);
        
        let mut editing_session = CommonEditingSession::new(point_history, static_context, context_history);
        editing_session.update_inference_context(inference_context);

        self.image_repo.save(image);
        self.editing_session_repo.save(&session_id, editing_session);

        let output = UploadOutput::new(session_id, image_id);

        Ok(output)
    }
}
