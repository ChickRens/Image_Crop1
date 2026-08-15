#[cfg(test)]
mod undo_redo_usecase_test {
    use std::{fs, path::Path};

    use crate::{
        application::{
            interface::{editing_session_repository::repository::EditingSessionRepository, image_loader::loader::ImageLoader, image_segmenter::segmenter::ImageSegmenter, rendered_image_cache::cache::RenderedImageCache}, types::{editing_session::session::{CommonEditingSession, EditingSession}, inference_context_history::InferenceContextHistory, point_history::PointHistory, rendered_image::RenderedImage}, usecase::{redo_usecase::{redo_input::RedoInput, usecase::RedoUseCase}, segment_usecase::{segment_input::SegmentInput, usecase::SegmentUseCase}, undo_usecase::{undo_input::UndoInput, usecase::UndoUseCase}, upload_usecase::{upload_input::UploadInput, usecase::UploadUseCase}},
        }, domain::{
            entity::{image::image::Image, session::session::Session}, repository::{original_image_repository::repository::OriginalImageRepository, session_repository::repository::SessionRepository}, value_object::{coordinate::Coordinate, point::{Point, PointLabel}, session_id::session_id::SessionId},
        }, infrastructure::{
            cache::shared_rendered_image_cache::SharedRenderedImageCacheInMemory, image_loader::FileImageLoader, repository::{
                shared_editing_session_repository::SharedEditingSessionRepository,
                shared_image_repository::SharedOriginalImageRepository,
                shared_session_repository::SharedSessionRepository,
            }, segmenter::{sam2_data::SAM2InferenceContext, shared_sam2::SharedSAM2Segmenter},
        },
    };

    fn _set_up() -> (SharedSessionRepository, SharedOriginalImageRepository, SharedSAM2Segmenter, SharedEditingSessionRepository, SharedRenderedImageCacheInMemory, SessionId) {
        let loader = FileImageLoader::new();
        let session_repo = SharedSessionRepository::new();
        let image_repo = SharedOriginalImageRepository::new();
        let segmenter = SharedSAM2Segmenter::new("models").unwrap();
        let editing_session_repo = SharedEditingSessionRepository::new();
        let image_cache = SharedRenderedImageCacheInMemory::new();

        let image_jpg = fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Tumbler.jpg")).unwrap();

        let upload_usecase = UploadUseCase::new(session_repo.clone(), image_repo.clone(), loader, segmenter.clone(), editing_session_repo.clone(), image_cache.clone());
        let upload_input = UploadInput::new(image_jpg);
        let output = upload_usecase.execute(upload_input).unwrap();
        let (session_id, original_image_id) = output.into_session_id_and_image_id();

        let segment_usecase = SegmentUseCase::new(session_repo.clone(), image_repo.clone(), segmenter.clone(), editing_session_repo.clone(), image_cache.clone());
        let segment_input = SegmentInput::new(session_id, Point::new(Coordinate::new(30, 40), PointLabel::BACKGROUND));
        let output = segment_usecase.execute(segment_input).unwrap();
        let segmented_image_id = output.image_id();

        (session_repo, image_repo, segmenter, editing_session_repo, image_cache, session_id)
    }

    #[test]
    fn test_undo_execute_returns_segmented_image() {
        let (session_repo, image_repo, segmenter, editing_session_repo, image_cache, session_id) = _set_up();
        let usecase = UndoUseCase::new(session_repo.clone(), image_repo.clone(), segmenter.clone(), editing_session_repo.clone(), image_cache.clone());

        let input = UndoInput::new(session_id);
        let output = usecase.execute(input).unwrap();

        let session = session_repo.get(&session_id).unwrap();
        let original_image_id = session.image_id();
        let original_image = image_repo.get(original_image_id).unwrap();

        let output_image_id = output.image_id();
        let output_rendered_image = image_cache.take(output_image_id).unwrap();
        let (output_image_data, _, _) = output_rendered_image.into_data();

        assert_eq!(*original_image.image_data(), output_image_data);
    }

    #[test]
    fn test_undo_redo_execute_returns_segmented_image() {
        let (session_repo, image_repo, segmenter, editing_session_repo, image_cache, session_id) = _set_up();
        let undo_usecase = UndoUseCase::new(session_repo.clone(), image_repo.clone(), segmenter.clone(), editing_session_repo.clone(), image_cache.clone());
        let redo_usecase = RedoUseCase::new(session_repo.clone(), image_repo.clone(), segmenter.clone(), editing_session_repo.clone(), image_cache.clone());

        let input = UndoInput::new(session_id);
        let _output = undo_usecase.execute(input).unwrap();

        let input = RedoInput::new(session_id);
        let output = redo_usecase.execute(input).unwrap();

        let id = output.image_id();
        assert!(image_cache.take(id).is_ok())
    }

    #[test]
    fn test_undo_with_missing_session_returns_error() {
        let (session_repo, image_repo, segmenter, editing_session_repo, image_cache, _) = _set_up();
        let usecase = UndoUseCase::new(session_repo, image_repo, segmenter, editing_session_repo, image_cache);

        let input = UndoInput::new(SessionId::new());
        let result = usecase.execute(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_undo_with_missing_image_returns_error() {
        let (session_repo, _, segmenter, editing_session_repo, image_cache, session_id) = _set_up();
        let image_repo = SharedOriginalImageRepository::new();
        let usecase = UndoUseCase::new(session_repo, image_repo, segmenter, editing_session_repo, image_cache);

        let input = UndoInput::new(session_id);
        let result = usecase.execute(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_undo_with_missing_editing_session_returns_error() {
        let (session_repo, image_repo, segmenter, _, image_cache, session_id) = _set_up();
        let editing_session_repo = SharedEditingSessionRepository::new();
        let usecase = UndoUseCase::new(session_repo, image_repo, segmenter, editing_session_repo, image_cache);

        let input = UndoInput::new(session_id);
        let result = usecase.execute(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_redo_without_prior_undo_returns_error() {
        let (session_repo, image_repo, segmenter, editing_session_repo, image_cache, session_id) = _set_up();
        let usecase = RedoUseCase::new(session_repo, image_repo, segmenter, editing_session_repo, image_cache);

        let input = RedoInput::new(session_id);
        let result = usecase.execute(input);

        assert!(result.is_err());
    }
}
