use crate::{application::{interface::{editing_session_repository::repository::EditingSessionRepository, image_segmenter::segmenter::ImageSegmenter, segmenter_input_image_generator::SegmenterInputImageGenerator, segmenter_input_image_storage::storage::SegmenterInputImageStorage}, service::prepare_service::PrepareSegmentService, usecase::{prepare_segment_usecase::{error::PrepareSegmentUseCaseError, prepare_segment_input::PrepareSegmentInput}}}, domain::repository::original_image_repository::repository::OriginalImageRepository};

pub struct PrepareSegmentUseCase<IR, IS, ESR, SS, SG>
where
    IR: OriginalImageRepository,
    IS: ImageSegmenter,
    ESR: EditingSessionRepository<
            StaticContext = IS::StaticContext,
            InferenceContext = IS::InferenceContext,
        >,
    SS: SegmenterInputImageStorage,
    SG: SegmenterInputImageGenerator,
{
    prepare_service: PrepareSegmentService<IS, IR, ESR, SG, SS>
}

impl<IR, IS, ESR, SS, SG> PrepareSegmentUseCase<IR, IS, ESR, SS, SG>
where
    IR: OriginalImageRepository,
    IS: ImageSegmenter,
    ESR: EditingSessionRepository<
            StaticContext = IS::StaticContext,
            InferenceContext = IS::InferenceContext,
        >,
    SS: SegmenterInputImageStorage,
    SG: SegmenterInputImageGenerator,
{
    pub fn new(
        prepare_service: PrepareSegmentService<IS, IR, ESR, SG, SS>
    ) -> Self {
        Self {
            prepare_service
        }
    }

    pub fn execute(&self, input: PrepareSegmentInput) -> Result<(), PrepareSegmentUseCaseError> {
        let image_id = input.image_id();
        let session_id = input.session_id();
        let preview_to_original_scale = input.point_scale();

        self.prepare_service.prepare(session_id, image_id, preview_to_original_scale)?;
        Ok(())
    }
}