use crate::{
    application::{
        errors::{application_errors::ApplicationErrors, segmentation_error::SegmentationErrors},
        usecase::{
            get_image_usecase::{
                get_image_input::GetImageInput, get_image_output::GetImageOutput,
                usecase::GetImageUseCase,
            },
            redo_usecase::{redo_input::RedoInput, redo_output::RedoOutput, usecase::RedoUseCase},
            segment_usecase::{
                segment_input::SegmentInput, segment_output::SegmentOutput, usecase::SegmentUseCase,
            },
            undo_usecase::{undo_input::UndoInput, undo_output::UndoOutput, usecase::UndoUseCase},
            upload_usecase::{
                upload_input::UploadInput, upload_output::UploadOutput, usecase::UploadUseCase,
            },
        },
    },
    infrastructure::{
        image_loader::FileImageLoader,
        repository::{
            shared_editing_session_repository::SharedEditingSessionRepository,
            shared_image_repository::SharedImageRepository,
            shared_session_repository::SharedSessionRepository,
        },
        segmenter::shared_sam2::SharedSAM2Segmenter,
    },
};

pub struct App {
    upload_usecase: UploadUseCase<
        SharedSessionRepository,
        SharedImageRepository,
        FileImageLoader,
        SharedSAM2Segmenter,
        SharedEditingSessionRepository,
    >,
    segment_usecase: SegmentUseCase<
        SharedSessionRepository,
        SharedImageRepository,
        SharedSAM2Segmenter,
        SharedEditingSessionRepository,
    >,
    undo_usecase: UndoUseCase<
        SharedSessionRepository,
        SharedImageRepository,
        SharedSAM2Segmenter,
        SharedEditingSessionRepository,
    >,
    redo_usecase: RedoUseCase<
        SharedSessionRepository,
        SharedImageRepository,
        SharedSAM2Segmenter,
        SharedEditingSessionRepository,
    >,
    get_image_usecase: GetImageUseCase<SharedSessionRepository, SharedImageRepository>,
}

impl App {
    pub fn new() -> Result<Self, ApplicationErrors> {
        let session_repo = SharedSessionRepository::new();
        let image_repo = SharedImageRepository::new();
        let loader = FileImageLoader::new();
        let segmenter = SharedSAM2Segmenter::new("models")
            .map_err(|err| SegmentationErrors::InitializeError(err.to_string()))?;
        let editing_session_repo = SharedEditingSessionRepository::new();

        let upload_uc = UploadUseCase::new(
            session_repo.clone(),
            image_repo.clone(),
            loader,
            segmenter.clone(),
            editing_session_repo.clone(),
        );
        let segment_uc = SegmentUseCase::new(
            session_repo.clone(),
            image_repo.clone(),
            segmenter.clone(),
            editing_session_repo.clone(),
        );
        let undo_uc = UndoUseCase::new(
            session_repo.clone(),
            image_repo.clone(),
            segmenter.clone(),
            editing_session_repo.clone(),
        );
        let redo_uc = RedoUseCase::new(
            session_repo.clone(),
            image_repo.clone(),
            segmenter.clone(),
            editing_session_repo.clone(),
        );
        let get_image_uc = GetImageUseCase::new(session_repo.clone(), image_repo.clone());

        Ok(App {
            upload_usecase: upload_uc,
            segment_usecase: segment_uc,
            undo_usecase: undo_uc,
            redo_usecase: redo_uc,
            get_image_usecase: get_image_uc,
        })
    }

    pub fn upload(&mut self, input: UploadInput) -> Result<UploadOutput, ApplicationErrors> {
        self.upload_usecase.execute(input)
    }

    pub fn segment(&mut self, input: SegmentInput) -> Result<SegmentOutput, ApplicationErrors> {
        self.segment_usecase.execute(input)
    }

    pub fn undo(&mut self, input: UndoInput) -> Result<UndoOutput, ApplicationErrors> {
        self.undo_usecase.execute(input)
    }

    pub fn redo(&mut self, input: RedoInput) -> Result<RedoOutput, ApplicationErrors> {
        self.redo_usecase.execute(input)
    }

    pub fn get_image(&mut self, input: GetImageInput) -> Result<GetImageOutput, ApplicationErrors> {
        self.get_image_usecase.execute(input)
    }
}
