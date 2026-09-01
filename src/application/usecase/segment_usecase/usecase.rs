use crate::{
    application::{
        interface::{
            editing_session_repository::repository::EditingSessionRepository,
            image_segmenter::segmenter::ImageSegmenter,
            preview_image_generator::PreviewImageGenerator,
            preview_storage::storage::PreviewStorage,
            segmenter_input_image_storage::storage::SegmenterInputImageStorage,
        }, service::{preview_service::PreviewService, segment_service::SegmentService}, usecase::segment_usecase::{
            error::SegmentUseCaseError,
            segment_input::SegmentInput,
            segment_output::SegmentOutput,
        },
    }, domain::{
        repository::{
            original_image_repository::repository::OriginalImageRepository,
            session_repository::repository::SessionRepository,
        },
    },
};

pub struct SegmentUseCase<SR, IR, IS, ESR, PS, PG, SS>
where
    SR: SessionRepository,
    IR: OriginalImageRepository,
    IS: ImageSegmenter,
    ESR: EditingSessionRepository<
            StaticContext = IS::StaticContext,
            InferenceContext = IS::InferenceContext,
        >,
    PS: PreviewStorage,
    PG: PreviewImageGenerator,
    SS: SegmenterInputImageStorage,
{
    segment_service: SegmentService<SR, IS, IR, ESR, SS>,
    preview_service: PreviewService<PG, PS>,
}

impl<SR, IR, IS, ESR, PS, PG, SS> SegmentUseCase<SR, IR, IS, ESR, PS, PG, SS>
where
    SR: SessionRepository,
    IR: OriginalImageRepository,
    IS: ImageSegmenter,
    ESR: EditingSessionRepository<
            StaticContext = IS::StaticContext,
            InferenceContext = IS::InferenceContext,
        >,
    PS: PreviewStorage,
    PG: PreviewImageGenerator,
    SS: SegmenterInputImageStorage,
{
    pub fn new(
        segment_service: SegmentService<SR, IS, IR, ESR, SS>,
        preview_service: PreviewService<PG, PS>,
    ) -> Self {
        Self {
            segment_service,
            preview_service,
        }
    }

    pub fn execute(&self, input: SegmentInput) -> Result<SegmentOutput, SegmentUseCaseError> {
        let (session_id, point) = input.into_parts();

        let segmented_image = self.segment_service.segment(session_id, point)?;
        self.preview_service.generate_and_save(&segmented_image);

        let segmented_image_id = *segmented_image.image_id();
        let output = SegmentOutput::new(segmented_image_id);
        Ok(output)
    }
}
