use crate::{
    application::usecase::{
        get_image_usecase::error::GetImageUseCaseError,
        prepare_segment_usecase::error::PrepareSegmentUseCaseError,
        redo_usecase::error::RedoUseCaseError, segment_usecase::error::SegmentUseCaseError,
        undo_usecase::error::UndoUseCaseError, upload_usecase::error::UploadUseCaseError,
    },
    parent_error,
};

parent_error!(
    pub enum ApplicationError {
        GetImageUseCase(GetImageUseCaseError),
        RedoUseCase(RedoUseCaseError),
        SegmentUseCase(SegmentUseCaseError),
        UndoUseCase(UndoUseCaseError),
        UploadUseCase(UploadUseCaseError),
        PrepareSegmentUseCase(PrepareSegmentUseCaseError),
    }
);
