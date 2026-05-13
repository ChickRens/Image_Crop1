use crate::application::errors::application_errors::ApplicationErrors;
use crate::application::errors::repository_errors::RepositoryErrors;
use crate::application::interface::image_segmenter::ImageSegmenter;
use crate::application::usecase::segment_usecase::segment_input::SegmentInput;
use crate::application::usecase::segment_usecase::segment_output::SegmentOutput;
use crate::domain::repository::image_repository::ImageRepository;
use crate::domain::repository::session_repository::SessionRepository;
use crate::domain::value_object::image_kind::ImageKind;

pub struct SegmentUseCase<SR, IR, IS>
where
    SR: SessionRepository,
    IR: ImageRepository,
    IS: ImageSegmenter,
{
    session_repo: SR,
    image_repo: IR,
    segmenter: IS,
}

impl<SR, IR, IS> SegmentUseCase<SR, IR, IS>
where
    SR: SessionRepository,
    IR: ImageRepository,
    IS: ImageSegmenter,
{
    pub fn new(session_repository: SR, image_repository: IR, image_segmenter: IS) -> Self {
        Self {
            session_repo: session_repository,
            image_repo: image_repository,
            segmenter: image_segmenter,
        }
    }

    pub fn execute(&mut self, input: SegmentInput) -> Result<SegmentOutput, ApplicationErrors> {
        let (session_id, image_id, points) = input.into_parts();
        let image = self.image_repo.get(&image_id, ImageKind::Original).ok_or(
            ApplicationErrors::RepositoryError(RepositoryErrors::ImageNotFound),
        )?;

        let segmented_image = self.segmenter.segment(image, &points);
        let segmented_image_data = segmented_image?.into_image();
        self.image_repo
            .save(segmented_image_data, ImageKind::Segmented);

        let output = SegmentOutput::new(session_id, image_id);
        Ok(output)
    }
}
