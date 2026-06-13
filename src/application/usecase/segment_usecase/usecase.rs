use crate::application::errors::application_errors::ApplicationErrors;
use crate::application::errors::repository_errors::RepositoryErrors;
use crate::application::interface::editing_session_repository::EditingSessionRepository;
use crate::application::interface::image_segmenter::ImageSegmenter;
use crate::application::types::editing_session::EditingSession;
use crate::application::usecase::segment_usecase::segment_input::SegmentInput;
use crate::application::usecase::segment_usecase::segment_output::SegmentOutput;
use crate::domain::entity::image::Image;
use crate::domain::repository::image_repository::ImageRepository;
use crate::domain::repository::session_repository::SessionRepository;
use crate::domain::value_object::image_id::ImageId;
use crate::domain::value_object::image_kind::ImageKind;

pub struct SegmentUseCase<SR, IR, IS, ESR>
where
    SR: SessionRepository,
    IR: ImageRepository,
    IS: ImageSegmenter,
    ESR: EditingSessionRepository<
            StaticContext = IS::StaticContext,
            InferenceContext = IS::InferenceContext,
        >,
{
    session_repo: SR,
    image_repo: IR,
    segmenter: IS,
    editing_session_repo: ESR,
}

impl<SR, IR, IS, ESR> SegmentUseCase<SR, IR, IS, ESR>
where
    SR: SessionRepository,
    IR: ImageRepository,
    IS: ImageSegmenter,
    ESR: EditingSessionRepository<
            StaticContext = IS::StaticContext,
            InferenceContext = IS::InferenceContext,
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

    pub fn execute(&mut self, input: SegmentInput) -> Result<SegmentOutput, ApplicationErrors> {
        let (session_id, image_id, points) = input.into_parts();
        let original_image = self.image_repo.get(&image_id, ImageKind::Original).ok_or(
            ApplicationErrors::RepositoryError(RepositoryErrors::ImageNotFound),
        )?;

        let editing_session = self.editing_session_repo.get_mut(&session_id).ok_or(
            ApplicationErrors::RepositoryError(RepositoryErrors::SessionNotFound),
        )?;

        editing_session.update_points(points);

        let segmented_image = self.segmenter.segment(&original_image, editing_session)?;
        let (segmented_image_data, size) = segmented_image.into_image_and_size();
        let segmented_image_id = ImageId::new();

        self.image_repo.save(
            Image::new(segmented_image_data, segmented_image_id, size),
            ImageKind::Segmented,
        );

        let output = SegmentOutput::new(session_id, segmented_image_id);
        Ok(output)
    }
}
