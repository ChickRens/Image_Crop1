use crate::{
    application::{
        interface::{
            editing_session_repository::repository::EditingSessionRepository, image_segmenter::segmenter::ImageSegmenter, preview_image_generator::PreviewImageGenerator, preview_storage::storage::PreviewStorage,
        }, types::editing_session::session::EditingSession, usecase::redo_usecase::{
            error::RedoUseCaseError, redo_input::RedoInput, redo_output::RedoOutput,
        },
    }, domain::{
        entity::image::Image, repository::{
            original_image_repository::repository::OriginalImageRepository,
            session_repository::repository::SessionRepository,
        }, value_object::image_id::image_id::ImageId,
    },
};

pub struct RedoUseCase<SR, IR, IS, ESR, PS, PG>
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
{
    session_repo: SR,
    image_repo: IR,
    segmenter: IS,
    editing_session_repo: ESR,
    preview_storage: PS,
    preview_generator: PG,
}

impl<SR, IR, IS, ESR, PS, PG> RedoUseCase<SR, IR, IS, ESR, PS, PG>
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
{
    pub fn new(
        session_repository: SR,
        image_repository: IR,
        image_segmenter: IS,
        editing_session_repository: ESR,
        preview_storage: PS,
        preview_generator: PG
    ) -> Self {
        Self {
            session_repo: session_repository,
            image_repo: image_repository,
            segmenter: image_segmenter,
            editing_session_repo: editing_session_repository,
            preview_storage: preview_storage,
            preview_generator: preview_generator,
        }
    }

    pub fn execute(&self, redo_input: RedoInput) -> Result<RedoOutput, RedoUseCaseError> {
        let session_id = redo_input.session_id();
        let session = self.session_repo.get(&session_id)?;

        let original_image_id = session.image_id();
        let original_image = self.image_repo.get(original_image_id)?;

        let mut editing_session = self.editing_session_repo.get(&session_id)?;

        editing_session.redo()?;
        let points = editing_session.points();
        let static_context = editing_session.static_context();
        let inference_context = editing_session.inference_context();

        let (_, segmented_image) =
            self.segmenter
                .segment(&original_image, static_context, inference_context, points)?;

        self.editing_session_repo.save(&session_id, editing_session);

        let (segmented_image_data, size) = segmented_image.into_image_and_size();
        let segmented_image_id = ImageId::new();
        let segmenter_image = Image::new(segmented_image_data, segmented_image_id, size);

        let preview_image = self.preview_generator.generate(&segmenter_image);
        self.preview_storage.save(preview_image);

        let output = RedoOutput::new(segmented_image_id);
        Ok(output)
    }
}
