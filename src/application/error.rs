use crate::{
    application::usecase::{
        get_preview_usecase::error::GetPreviewUseCaseError,
        prepare_segment_usecase::error::PrepareSegmentUseCaseError,
        redo_usecase::error::RedoUseCaseError, segment_usecase::error::SegmentUseCaseError,
        undo_usecase::error::UndoUseCaseError, upload_usecase::error::UploadUseCaseError,
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
        PrepareSegmentUseCase(PrepareSegmentUseCaseError),
    }
);
