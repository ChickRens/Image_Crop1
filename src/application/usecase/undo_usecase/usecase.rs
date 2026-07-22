use crate::{
    application::{
        errors::{application_errors::ApplicationErrors, repository_errors::RepositoryErrors},
        interface::{
            editing_session_repository::EditingSessionRepository, image_segmenter::ImageSegmenter,
        },
        types::editing_session::EditingSession,
        usecase::undo_usecase::{undo_input::UndoInput, undo_output::UndoOutput},
    },
    domain::{
        entity::image::Image,
        repository::{image_repository::ImageRepository, session_repository::SessionRepository},
        value_object::{image_id::ImageId, image_kind::ImageKind},
    },
};

pub struct UndoUseCase<SR, IR, IS, ESR>
where
    SR: SessionRepository,
    IS: ImageSegmenter,
    IR: ImageRepository,
    ESR: EditingSessionRepository<
            StaticContext = <IS as ImageSegmenter>::StaticContext,
            InferenceContext = <IS as ImageSegmenter>::InferenceContext,
        >,
{
    session_repo: SR,
    image_repo: IR,
    segmenter: IS,
    editing_session_repo: ESR,
}

impl<SR, IR, IS, ESR> UndoUseCase<SR, IR, IS, ESR>
where
    SR: SessionRepository,
    IR: ImageRepository,
    IS: ImageSegmenter,
    ESR: EditingSessionRepository<
            StaticContext = <IS as ImageSegmenter>::StaticContext,
            InferenceContext = <IS as ImageSegmenter>::InferenceContext,
        >,
{
    pub fn new(
        session_repository: SR,
        image_repository: IR,
        image_segmenter: IS,
        editing_session_repository: ESR,
    ) -> Self {
        Self {
            session_repo: session_repository,
            image_repo: image_repository,
            segmenter: image_segmenter,
            editing_session_repo: editing_session_repository,
        }
    }

    pub fn execute(&self, undo_input: UndoInput) -> Result<UndoOutput, ApplicationErrors> {
        let session_id = undo_input.session_id();
        let session =
            self.session_repo
                .get(&session_id)
                .ok_or(ApplicationErrors::RepositoryError(
                    RepositoryErrors::SessionNotFound,
                ))?;

        let image_id = session.image_id();
        let image = self.image_repo.get(image_id, ImageKind::Original).ok_or(
            ApplicationErrors::RepositoryError(RepositoryErrors::ImageNotFound),
        )?;

        let mut editing_session = self.editing_session_repo.get(&session_id).ok_or(
            ApplicationErrors::RepositoryError(RepositoryErrors::SessionNotFound),
        )?;

        editing_session.undo();
        let points = editing_session.points();
        let static_context = editing_session.static_context();

        let (new_context, segmented_image) =
            self.segmenter.rebuild(&image, static_context, points)?;
        editing_session.set_inference_context(new_context);

        let (segmented_image_data, size) = segmented_image.into_image_and_size();
        let segmented_image_id = ImageId::new();

        self.editing_session_repo.save(&session_id, editing_session);

        self.image_repo.save(
            Image::new(segmented_image_data, segmented_image_id, size),
            ImageKind::Segmented,
        );

        let output = UndoOutput::new(session_id, segmented_image_id);
        Ok(output)
    }
}
