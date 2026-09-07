use crate::application::{service::{completed_service::CompletedService, segment_service::SegmentService}, usecase::save_usecase::{error::SaveUseCaseError, input::SaveInput, output::SaveOutput}};

pub struct SaveUseCase<SS, CS>
where
    SS: SegmentService,
    CS: CompletedService,
{
    segment_service: SS,
    completed_image_repository: CS,
}

impl<SS, CS> SaveUseCase<SS, CS>
where
    SS: SegmentService,
    CS: CompletedService,
{
    pub fn new(segment_service: SS, completed_image_repository: CS) -> Self {
        Self {
            segment_service,
            completed_image_repository,
        }
    }

    pub fn execute(&self, save_input: SaveInput) -> Result<SaveOutput, SaveUseCaseError> {
        let session_id = save_input.session_id();

        let segmented_image = self.segment_service.resegment(session_id)?;
        let segmented_image_id = *segmented_image.image_id();

        let completed_image = self.completed_image_repository.generate_and_save(segmented_image);
        self.completed_image_repository.save(completed_image);
        
        let output = SaveOutput::new(segmented_image_id);
        Ok(output)
    }
}
