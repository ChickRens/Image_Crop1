use crate::application::errors::repository_errors::RepositoryErrors;
use crate::application::interface::image_segmenter::ImageSegmenter;
use crate::application::usecase::segment_usecase::segment_input::SegmentInput;
use crate::domain::repository::session_repository::SessionRepository;
use crate::domain::repository::image_meta_repository::ImageMetaRepository;
use crate::application::repository::image_repository::ImageRepository;
use crate::application::errors::application_errors::ApplicationErrors;

pub struct SegmentUseCase<SR, IR, MR, IS>
where
    SR: SessionRepository,
    IR: ImageRepository,
    MR: ImageMetaRepository,
    IS: ImageSegmenter
{
    session_repo: SR,
    image_repo: IR,
    meta_repo: MR,
    segmenter: IS,
}

impl<SR, IR, MR, IS> SegmentUseCase<SR, IR, MR, IS>
where
    SR: SessionRepository,
    IR: ImageRepository,
    MR: ImageMetaRepository,
    IS: ImageSegmenter,
{
    pub fn new(
        session_repository: SR,
        image_repository: IR,
        image_meta_repository: MR,
        image_segmenter: IS,
    ) -> Self {
        Self {
            session_repo: session_repository,
            image_repo: image_repository,
            meta_repo: image_meta_repository,
            segmenter: image_segmenter,
        }
    }

    pub fn execute(&mut self, input: SegmentInput) -> Result<(), ApplicationErrors> {
        let (session_id, image_id, points) = input.into_parts();
        let image = self.image_repo.get(&image_id)
                            .ok_or(ApplicationErrors::RepositoryError(RepositoryErrors::ImageNotFound))?;

        let segmented_image= self.segmenter.segment(image, points.as_ref());
        Ok(())
    }}