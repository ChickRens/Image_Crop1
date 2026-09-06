use crate::{application::{service::segment_service::SegmentService, usecase::save_usecase::{error::SaveUseCaseError, input::SaveInput, output::SaveOutput}}, domain::{entity::completed_image::CompletedImage, repository::completed_image_repository::repository::CompletedImageRepository}};

pub struct SaveUseCase<SS, CR>
where
    SS: SegmentService,
    CR: CompletedImageRepository,
{
    segment_service: SS,
    completed_image_repository: CR,
}

impl<SS, CR> SaveUseCase<SS, CR>
where
    SS: SegmentService,
    CR: CompletedImageRepository,
{
    pub fn new(segment_service: SS, completed_image_repository: CR) -> Self {
        Self {
            segment_service,
            completed_image_repository,
        }
    }

    pub fn execute(&self, save_input: SaveInput) -> Result<SaveOutput, SaveUseCaseError> {
        let session_id = save_input.session_id();

        let segmented_image = self.segment_service.resegment(session_id)?;
        let segmented_image_id = *segmented_image.image_id();

        let completed_image = CompletedImage::new(segmented_image);
        self.completed_image_repository.save(completed_image);
        
        let output = SaveOutput::new(segmented_image_id);
        Ok(output)
    }
}
