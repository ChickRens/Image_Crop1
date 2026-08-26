use crate::{application::{interface::{editing_session_repository::repository::EditingSessionRepository, image_segmenter::segmenter::ImageSegmenter}, types::{editing_session::session::CommonEditingSession, inference_context_history::InferenceContextHistory, point_history::PointHistory}, usecase::{config::MAX_HISTORY, prepare_segment_usecase::{error::PrepareSegmentUseCaseError, prepare_segment_input::PrepareSegmentInput}}}, domain::repository::original_image_repository::repository::OriginalImageRepository};

pub struct PrepareSegmentUseCase<IR, IS, ESR>
where
    IR: OriginalImageRepository,
    IS: ImageSegmenter,
    ESR: EditingSessionRepository<
            StaticContext = IS::StaticContext,
            InferenceContext = IS::InferenceContext,
        >,
{
    image_repo: IR,
    segmenter: IS,
    editing_session_repo: ESR,
}

impl<IR, IS, ESR> PrepareSegmentUseCase<IR, IS, ESR>
where
    IR: OriginalImageRepository,
    IS: ImageSegmenter,
    ESR: EditingSessionRepository<
            StaticContext = IS::StaticContext,
            InferenceContext = IS::InferenceContext,
        >,
{
    pub fn new(
        image_repository: IR,
        image_segmenter: IS,
        editing_session_repository: ESR,
    ) -> Self {
        Self {
            image_repo: image_repository,
            segmenter: image_segmenter,
            editing_session_repo: editing_session_repository,
        }
    }

    pub fn execute(&self, input: PrepareSegmentInput) -> Result<(), PrepareSegmentUseCaseError> {
        let image_id = input.image_id();
        let session_id = input.session_id();

        let original_image = self.image_repo.get(&image_id)?;

        let static_context = self.segmenter.prepare_static_context(&original_image)?;
        let inference_context = self.segmenter.prepare_inference_context(&original_image)?;

        let point_history = PointHistory::new(MAX_HISTORY);
        let inference_context_history = InferenceContextHistory::new(MAX_HISTORY);

        let editing_session = CommonEditingSession::new(point_history, static_context, inference_context_history, inference_context);
        self.editing_session_repo.save(&session_id, editing_session);

        Ok(())
    }
}