use crate::{
    application::{
        interface::{
            editing_session_repository::repository::EditingSessionRepository, image_segmenter::segmenter::ImageSegmenter, preview_image_generator::PreviewImageGenerator, preview_storage::storage::PreviewStorage, segmenter_input_image_storage::storage::SegmenterInputImageStorage,
        }, types::editing_session::session::EditingSession, usecase::segment_usecase::{
            error::SegmentUseCaseError, segment_input::SegmentInput, segment_output::SegmentOutput,
        },
    }, domain::{
        entity::image::Image, repository::{
            session_repository::repository::SessionRepository,
        }, value_object::image_id::image_id::ImageId,
    },
};

pub struct SegmentUseCase<SR, IS, ESR, PS, PG, SS>
where
    SR: SessionRepository,
    IS: ImageSegmenter,
    ESR: EditingSessionRepository<
            StaticContext = IS::StaticContext,
            InferenceContext = IS::InferenceContext,
        >,
    PS: PreviewStorage,
    PG: PreviewImageGenerator,
    SS: SegmenterInputImageStorage,
{
    session_repo: SR,
    segmenter: IS,
    editing_session_repo: ESR,
    preview_storage: PS,
    preview_generator: PG,
    input_storage: SS,
}

impl<SR, IS, ESR, PS, PG, SS> SegmentUseCase<SR, IS, ESR, PS, PG, SS>
where
    SR: SessionRepository,
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
        session_repository: SR,
        image_segmenter: IS,
        editing_session_repository: ESR,
        preview_storage: PS,
        preview_generator: PG,
        segmenter_input_storage: SS,
    ) -> Self {
        Self {
            session_repo: session_repository,
            segmenter: image_segmenter,
            editing_session_repo: editing_session_repository,
            preview_storage: preview_storage,
            preview_generator: preview_generator,
            input_storage: segmenter_input_storage,
        }
    }

    pub fn execute(&self, input: SegmentInput) -> Result<SegmentOutput, SegmentUseCaseError> {
        let (session_id, point) = input.into_parts();
        let session = self.session_repo.get(&session_id)?;
        let image_id = session.image_id();

        let original_input_image = self.input_storage.get(*image_id)?;

        let mut editing_session = self.editing_session_repo.get(&session_id)?;

        let static_context = editing_session.static_context();
        let inference_context = editing_session.inference_context();
        let points = &editing_session.points_with(point.clone());

        let (new_inference_context, segmented_image) =
            self.segmenter
                .segment(&original_input_image, static_context, inference_context, points)?;

        let (segmented_image_data, size) = segmented_image.into_image_and_size();
        let segmented_image_id = ImageId::new();
        let segmented_image = Image::new(segmented_image_data, segmented_image_id, size);

        editing_session.apply_edit(point, new_inference_context);
        self.editing_session_repo.save(&session_id, editing_session);

        let preview_image = self.preview_generator.generate(&segmented_image);
        self.preview_storage.save(preview_image);

        let output = SegmentOutput::new(segmented_image_id);
        Ok(output)
    }
}
