use crate::{
    application::usecase::{
        get_completed_usecase::error::GetCompletedUseCaseError,
        get_preview_usecase::error::GetPreviewUseCaseError,
        prepare_segment_usecase::error::PrepareSegmentUseCaseError,
        redo_usecase::error::RedoUseCaseError, save_usecase::error::SaveUseCaseError,
        segment_usecase::error::SegmentUseCaseError, undo_usecase::error::UndoUseCaseError,
        upload_usecase::error::UploadUseCaseError,
    },
    parent_error,
};

parent_error!(
    pub enum ApplicationError {
        GetPreviewUseCase(GetPreviewUseCaseError),
        RedoUseCase(RedoUseCaseError),
        SegmentUseCase(SegmentUseCaseError),
        UndoUseCase(UndoUseCaseError),
        UploadUseCase(UploadUseCaseError),
        SaveUseCase(SaveUseCaseError),
        GetCompletedUseCase(GetCompletedUseCaseError),
        PrepareSegmentUseCase(PrepareSegmentUseCaseError),
    }
);
