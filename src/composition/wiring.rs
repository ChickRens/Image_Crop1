use crate::{
    application::{
        error::ApplicationError,
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
        cache::shared_rendered_image_cache::SharedRenderedImageCacheInMemory,
        image_loader::FileImageLoader,
        repository::{
            shared_editing_session_repository::SharedEditingSessionRepository,
            shared_image_repository::SharedOriginalImageRepository,
            shared_session_repository::SharedSessionRepository,
        },
        segmenter::shared_sam2::SharedSAM2Segmenter,
    },
};

pub struct App {
    upload_usecase: UploadUseCase<
        SharedSessionRepository,
        SharedOriginalImageRepository,
        FileImageLoader,
        SharedSAM2Segmenter,
        SharedEditingSessionRepository,
        SharedRenderedImageCacheInMemory,
    >,
    segment_usecase: SegmentUseCase<
        SharedSessionRepository,
        SharedOriginalImageRepository,
        SharedSAM2Segmenter,
        SharedEditingSessionRepository,
        SharedRenderedImageCacheInMemory,
    >,
    undo_usecase: UndoUseCase<
        SharedSessionRepository,
        SharedOriginalImageRepository,
        SharedSAM2Segmenter,
        SharedEditingSessionRepository,
        SharedRenderedImageCacheInMemory,
    >,
    redo_usecase: RedoUseCase<
        SharedSessionRepository,
        SharedOriginalImageRepository,
        SharedSAM2Segmenter,
        SharedEditingSessionRepository,
        SharedRenderedImageCacheInMemory,
    >,
    get_image_usecase: GetImageUseCase<SharedRenderedImageCacheInMemory>,
}

impl App {
    pub fn new() -> Result<Self, ApplicationError> {
        let session_repo = SharedSessionRepository::new();
        let image_repo = SharedOriginalImageRepository::new();
        let loader = FileImageLoader::new();
        let segmenter = SharedSAM2Segmenter::new("models")
            .expect("Failed to load SAM2 model during app initialize");
        let editing_session_repo = SharedEditingSessionRepository::new();
        let image_cache = SharedRenderedImageCacheInMemory::new();

        let upload_uc = UploadUseCase::new(
            session_repo.clone(),
            image_repo.clone(),
            loader,
            segmenter.clone(),
            editing_session_repo.clone(),
            image_cache.clone(),
        );
        let segment_uc = SegmentUseCase::new(
            session_repo.clone(),
            image_repo.clone(),
            segmenter.clone(),
            editing_session_repo.clone(),
            image_cache.clone(),
        );
        let undo_uc = UndoUseCase::new(
            session_repo.clone(),
            image_repo.clone(),
            segmenter.clone(),
            editing_session_repo.clone(),
            image_cache.clone(),
        );
        let redo_uc = RedoUseCase::new(
            session_repo.clone(),
            image_repo.clone(),
            segmenter.clone(),
            editing_session_repo.clone(),
            image_cache.clone(),
        );
        let get_image_uc = GetImageUseCase::new(image_cache.clone());

        Ok(App {
            upload_usecase: upload_uc,
            segment_usecase: segment_uc,
            undo_usecase: undo_uc,
            redo_usecase: redo_uc,
            get_image_usecase: get_image_uc,
        })
    }

    pub fn upload(&self, input: UploadInput) -> Result<UploadOutput, ApplicationError> {
        Ok(self.upload_usecase.execute(input)?)
    }

    pub fn segment(&self, input: SegmentInput) -> Result<SegmentOutput, ApplicationError> {
        Ok(self.segment_usecase.execute(input)?)
    }

    pub fn undo(&self, input: UndoInput) -> Result<UndoOutput, ApplicationError> {
        Ok(self.undo_usecase.execute(input)?)
    }

    pub fn redo(&self, input: RedoInput) -> Result<RedoOutput, ApplicationError> {
        Ok(self.redo_usecase.execute(input)?)
    }

    pub fn get_image(&self, input: GetImageInput) -> Result<GetImageOutput, ApplicationError> {
        Ok(self.get_image_usecase.execute(input)?)
    }
}
