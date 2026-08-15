use crate::{
    application::{
        interface::{
            editing_session_repository::repository::EditingSessionRepository,
            image_segmenter::segmenter::ImageSegmenter,
            rendered_image_cache::cache::RenderedImageCache,
        },
        types::{editing_session::session::EditingSession, rendered_image::RenderedImage},
        usecase::redo_usecase::{
            error::RedoUseCaseError, redo_input::RedoInput, redo_output::RedoOutput,
        },
    },
    domain::{
        repository::{
            original_image_repository::repository::OriginalImageRepository,
            session_repository::repository::SessionRepository,
        },
        value_object::image_id::image_id::ImageId,
    },
};

pub struct RedoUseCase<SR, IR, IS, ESR, IC>
where
    SR: SessionRepository,
    IS: ImageSegmenter,
    IR: OriginalImageRepository,
    ESR: EditingSessionRepository<
            StaticContext = <IS as ImageSegmenter>::StaticContext,
            InferenceContext = <IS as ImageSegmenter>::InferenceContext,
        >,
    IC: RenderedImageCache,
{
    session_repo: SR,
    image_repo: IR,
    segmenter: IS,
    editing_session_repo: ESR,
    image_cache: IC,
}

impl<SR, IR, IS, ESR, IC> RedoUseCase<SR, IR, IS, ESR, IC>
where
    SR: SessionRepository,
    IR: OriginalImageRepository,
    IS: ImageSegmenter,
    ESR: EditingSessionRepository<
            StaticContext = <IS as ImageSegmenter>::StaticContext,
            InferenceContext = <IS as ImageSegmenter>::InferenceContext,
        >,
    IC: RenderedImageCache,
{
    pub fn new(
        session_repository: SR,
        image_repository: IR,
        image_segmenter: IS,
        editing_session_repository: ESR,
        rendered_image_cache: IC,
    ) -> Self {
        Self {
            session_repo: session_repository,
            image_repo: image_repository,
            segmenter: image_segmenter,
            editing_session_repo: editing_session_repository,
            image_cache: rendered_image_cache,
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

        let (_new_context, segmented_image) =
            self.segmenter
                .segment(&original_image, static_context, inference_context, points)?;

        let (segmented_image_data, size) = segmented_image.into_image_and_size();
        let segmented_image_id = ImageId::new();

        self.editing_session_repo.save(&session_id, editing_session);

        self.image_cache.save(RenderedImage::new(
            segmented_image_data,
            segmented_image_id,
            size,
        ));

        let output = RedoOutput::new(segmented_image_id);
        Ok(output)
    }
}
