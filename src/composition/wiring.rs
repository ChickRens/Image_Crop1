use crate::{
    application::{
        error::ApplicationError,
        service::{
            completed_service::SharedCompletedService,
            prepare_service::SharedPrepareSegmentService, preview_service::SharedPreviewService,
            segment_service::SharedSegmentService,
        },
        usecase::{
            get_completed_usecase::{
                input::GetCompletedInput, output::GetCompletedOutput, usecase::GetCompletedUseCase,
            },
            get_preview_usecase::{
                get_preview_input::GetPreviewInput, get_preview_output::GetPreviewOutput,
                usecase::GetPreviewUseCase,
            },
            prepare_segment_usecase::{
                prepare_segment_input::PrepareSegmentInput, usecase::PrepareSegmentUseCase,
            },
            redo_usecase::{redo_input::RedoInput, redo_output::RedoOutput, usecase::RedoUseCase},
            save_usecase::{input::SaveInput, output::SaveOutput, usecase::SaveUseCase},
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
        generator::{
            completed_image::SharedWebPCompletedImageGenerator,
            shared_preview::SharedPreviewGenerator,
            shared_segmenter_input::SharedSAM2InputGenerator,
        },
        image_loader::FileImageLoader,
        repository::{
            completed_repository::SharedCompletedImageRepository,
            shared_editing_session_repository::SharedEditingSessionRepository,
            shared_image_repository::SharedOriginalImageRepository,
            shared_session_repository::SharedSessionRepository,
        },
        segmenter::shared_sam2::SharedSAM2Segmenter,
        storage::{
            shared_preview_storage::SharedPreviewStorage,
            shared_segmenter_input_storage::SharedSegmenterInputStorage,
        },
    },
};

pub struct App {
    upload_usecase: UploadUseCase<
        SharedSessionRepository,
        SharedOriginalImageRepository,
        FileImageLoader,
        SharedPreviewService<SharedPreviewGenerator, SharedPreviewStorage>,
    >,
    prepare_segment_usecase: PrepareSegmentUseCase<
        SharedPrepareSegmentService<
            SharedSAM2Segmenter,
            SharedOriginalImageRepository,
            SharedEditingSessionRepository,
            SharedSAM2InputGenerator,
            SharedSegmenterInputStorage,
        >,
    >,
    segment_usecase: SegmentUseCase<
        SharedSegmentService<
            SharedSessionRepository,
            SharedSAM2Segmenter,
            SharedOriginalImageRepository,
            SharedEditingSessionRepository,
            SharedSegmenterInputStorage,
        >,
        SharedPreviewService<SharedPreviewGenerator, SharedPreviewStorage>,
    >,
    undo_usecase: UndoUseCase<
        SharedSegmentService<
            SharedSessionRepository,
            SharedSAM2Segmenter,
            SharedOriginalImageRepository,
            SharedEditingSessionRepository,
            SharedSegmenterInputStorage,
        >,
        SharedPreviewService<SharedPreviewGenerator, SharedPreviewStorage>,
    >,
    redo_usecase: RedoUseCase<
        SharedPreviewService<SharedPreviewGenerator, SharedPreviewStorage>,
        SharedSegmentService<
            SharedSessionRepository,
            SharedSAM2Segmenter,
            SharedOriginalImageRepository,
            SharedEditingSessionRepository,
            SharedSegmenterInputStorage,
        >,
    >,
    save_usecase: SaveUseCase<
        SharedSegmentService<
            SharedSessionRepository,
            SharedSAM2Segmenter,
            SharedOriginalImageRepository,
            SharedEditingSessionRepository,
            SharedSegmenterInputStorage,
        >,
        SharedCompletedService<SharedWebPCompletedImageGenerator, SharedCompletedImageRepository>,
    >,
    get_completed_usecase: GetCompletedUseCase<
        SharedCompletedService<SharedWebPCompletedImageGenerator, SharedCompletedImageRepository>,
    >,
    get_preview_usecase:
        GetPreviewUseCase<SharedPreviewService<SharedPreviewGenerator, SharedPreviewStorage>>,
}

impl App {
    pub fn new() -> Result<Self, ApplicationError> {
        let session_repo = SharedSessionRepository::new();
        let image_repo = SharedOriginalImageRepository::new();
        let loader = FileImageLoader::new();
        let segmenter = SharedSAM2Segmenter::new("models")
            .expect("Failed to load SAM2 model during app initialize");
        let editing_session_repo = SharedEditingSessionRepository::new();
        let preview_storage = SharedPreviewStorage::new();
        let preview_generator = SharedPreviewGenerator::new(600);
        let input_storage = SharedSegmenterInputStorage::new();
        let input_generator = SharedSAM2InputGenerator::new(1024, 1024);
        let completed_image_generator = SharedWebPCompletedImageGenerator::new();
        let completed_image_repository = SharedCompletedImageRepository::new();

        let preview_service = SharedPreviewService::new(preview_generator, preview_storage);
        let prepare_service = SharedPrepareSegmentService::new(
            segmenter.clone(),
            image_repo.clone(),
            editing_session_repo.clone(),
            input_generator,
            input_storage.clone(),
            50,
        );
        let segment_service = SharedSegmentService::new(
            session_repo.clone(),
            segmenter.clone(),
            image_repo.clone(),
            editing_session_repo.clone(),
            input_storage.clone(),
        );
        let completed_service =
            SharedCompletedService::new(completed_image_generator, completed_image_repository);

        let upload_uc = UploadUseCase::new(
            session_repo.clone(),
            image_repo.clone(),
            loader,
            preview_service.clone(),
        );
        let prepare_segment_uc = PrepareSegmentUseCase::new(prepare_service);
        let segment_uc = SegmentUseCase::new(segment_service.clone(), preview_service.clone());
        let undo_uc = UndoUseCase::new(segment_service.clone(), preview_service.clone());
        let redo_uc = RedoUseCase::new(segment_service.clone(), preview_service.clone());
        let save_uc = SaveUseCase::new(segment_service.clone(), completed_service.clone());
        let get_completed_uc = GetCompletedUseCase::new(completed_service.clone());
        let get_image_uc = GetPreviewUseCase::new(preview_service.clone());

        Ok(App {
            upload_usecase: upload_uc,
            prepare_segment_usecase: prepare_segment_uc,
            segment_usecase: segment_uc,
            undo_usecase: undo_uc,
            redo_usecase: redo_uc,
            get_completed_usecase: get_completed_uc,
            save_usecase: save_uc,
            get_preview_usecase: get_image_uc,
        })
    }

    pub fn upload(&self, input: UploadInput) -> Result<UploadOutput, ApplicationError> {
        let output = self.upload_usecase.execute(input)?;
        let preview_to_original_point_scale = output.scale();
        let (session_id, image_id) = output.session_id_and_image_id();

        let input = PrepareSegmentInput::new(image_id, session_id, preview_to_original_point_scale);
        self.prepare_segment_usecase.execute(input)?;
        Ok(output)
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

    pub fn save(&self, input: SaveInput) -> Result<SaveOutput, ApplicationError> {
        Ok(self.save_usecase.execute(input)?)
    }

    pub async fn get_completed_image(
        &self,
        input: GetCompletedInput,
    ) -> Result<GetCompletedOutput, ApplicationError> {
        Ok(self.get_completed_usecase.execute(input)?)
    }

    pub async fn get_preview(
        &self,
        input: GetPreviewInput,
    ) -> Result<GetPreviewOutput, ApplicationError> {
        Ok(self.get_preview_usecase.execute(input)?)
    }
}
