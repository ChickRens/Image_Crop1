use crate::{
    application::{
        interface::{
            editing_session_repository::repository::EditingSessionRepository,
            image_segmenter::segmenter::ImageSegmenter,
            preview_image_generator::PreviewImageGenerator,
            preview_storage::storage::PreviewStorage,
            segmenter_input_image_storage::storage::SegmenterInputImageStorage,
        }, service::{preview_service::PreviewService, segment_service::SegmentService}, usecase::undo_usecase::{
            error::UndoUseCaseError,
            undo_input::UndoInput,
            undo_output::UndoOutput,
        },
    }, domain::{
        repository::{
            original_image_repository::repository::OriginalImageRepository,
            session_repository::repository::SessionRepository,
        },
    },
};

pub struct UndoUseCase<SR, IR, IS, ESR, PS, PG, SS>
where
    SR: SessionRepository,
    IS: ImageSegmenter,
    IR: OriginalImageRepository,
    ESR: EditingSessionRepository<
            StaticContext = <IS as ImageSegmenter>::StaticContext,
            InferenceContext = <IS as ImageSegmenter>::InferenceContext,
        >,
    PS: PreviewStorage,
    PG: PreviewImageGenerator,
    SS: SegmenterInputImageStorage,
{
    segment_service: SegmentService<SR, IS, IR, ESR, SS>,
    preview_service: PreviewService<PG, PS>,
}

impl<SR, IR, IS, ESR, PS, PG, SS> UndoUseCase<SR, IR, IS, ESR, PS, PG, SS>
where
    SR: SessionRepository,
    IR: OriginalImageRepository,
    IS: ImageSegmenter,
    ESR: EditingSessionRepository<
            StaticContext = <IS as ImageSegmenter>::StaticContext,
            InferenceContext = <IS as ImageSegmenter>::InferenceContext,
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

    pub fn execute(&self, undo_input: UndoInput) -> Result<UndoOutput, UndoUseCaseError> {
        let session_id = undo_input.session_id();

        let segmented_image = self.segment_service.undo(session_id)?;
        self.preview_service.generate_and_save(&segmented_image);
        let segmented_image_id = *segmented_image.image_id();

        let output = UndoOutput::new(segmented_image_id);
        Ok(output)
    }
}
