#[cfg(test)]
mod segment_usecase_test {
    use std::path::Path;
    use std::fs::read;

use image::RgbaImage;

use crate::application::interface::image_loader::loader::ImageLoader;
use crate::application::interface::rendered_image_cache::cache::RenderedImageCache;
use crate::application::usecase::segment_usecase::error::SegmentUseCaseError;
use crate::application::usecase::segment_usecase::segment_input::SegmentInput;
use crate::application::usecase::segment_usecase::usecase::SegmentUseCase;
use crate::application::usecase::upload_usecase::upload_input::UploadInput;
use crate::application::usecase::upload_usecase::usecase::UploadUseCase;
use crate::domain::entity::session::session::Session;
use crate::domain::repository::original_image_repository::repository::OriginalImageRepository;
use crate::domain::repository::session_repository::error::SessionRepositoryError;
use crate::domain::repository::session_repository::repository::SessionRepository;
use crate::domain::value_object::coordinate::Coordinate;
use crate::domain::value_object::point::{Point, PointLabel};
use crate::domain::value_object::session_id::session_id::SessionId;
use crate::infrastructure::cache::shared_rendered_image_cache::SharedRenderedImageCacheInMemory;
use crate::infrastructure::image_loader::FileImageLoader;
use crate::infrastructure::repository::editing_session_repository::SAM2EditingSessionRepository;
use crate::infrastructure::repository::image_repository::OriginalImageRepositoryInMemory;
use crate::infrastructure::repository::session_repository::SessionRepositoryInMemory;
use crate::infrastructure::repository::shared_editing_session_repository::SharedEditingSessionRepository;
use crate::infrastructure::repository::shared_image_repository::SharedOriginalImageRepository;
use crate::infrastructure::repository::shared_session_repository::SharedSessionRepository;
use crate::infrastructure::segmenter::sam2::Sam2Segmenter;
use crate::infrastructure::segmenter::shared_sam2::SharedSAM2Segmenter;

    fn _set_up(image_jpg: Vec<u8>) -> (SharedSessionRepository, SharedOriginalImageRepository, SharedSAM2Segmenter, SharedEditingSessionRepository, SharedRenderedImageCacheInMemory, SessionId) {
        let model_dir = "models";
        let loader = FileImageLoader::new();
        let image_cache = SharedRenderedImageCacheInMemory::new();
        let session_repo = SharedSessionRepository::new();
        let image_repo = SharedOriginalImageRepository::new();
        let segmenter = SharedSAM2Segmenter::new(model_dir).expect("model load failed");
        let editing_session_repo = SharedEditingSessionRepository::new();

        let upload_usecase = UploadUseCase::new(session_repo.clone(), image_repo.clone(), loader, segmenter.clone(), editing_session_repo.clone(), image_cache.clone());
        let upload_input = UploadInput::new(image_jpg);
        let output = upload_usecase.execute(upload_input).unwrap();
        let (session_id, image_id) = output.into_session_id_and_image_id();

        (session_repo, image_repo, segmenter, editing_session_repo, image_cache, session_id)
    }


    #[test]
    fn test_normal_execute() {
        let image_jpg =
            read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Tumbler.jpg"))
                .unwrap();

        let (session_repo, image_repo, segmenter, editing_session_repo, image_cache, session_id) = _set_up(image_jpg);
        let usecase = SegmentUseCase::new(session_repo.clone(), image_repo.clone(), segmenter.clone(), editing_session_repo.clone(), image_cache.clone());

        let point = Point::new(Coordinate::new(30, 40), PointLabel::FOREGROUND);

        let input = SegmentInput::new(session_id, point);
        let output = usecase.execute(input).unwrap();
        let new_image_id = output.image_id();

        assert!(!new_image_id.value().to_string().is_empty());

        let result = image_cache.take(new_image_id);
        assert!(result.is_ok())
    }

    #[test]
    fn test_execute_with_missing_session_returns_error() {
        let image_jpg =
            read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Tumbler.jpg"))
                .unwrap();

        let (session_repo, image_repo, segmenter, editing_session_repo, image_cache, session_id) = _set_up(image_jpg);
        let usecase = SegmentUseCase::new(session_repo.clone(), image_repo.clone(), segmenter.clone(), editing_session_repo.clone(), image_cache.clone());

        let missing_session_id = SessionId::new();
        let point = Point::new(Coordinate::new(30, 40), PointLabel::FOREGROUND);
        let input = SegmentInput::new(missing_session_id, point);

        let result = usecase.execute(input);

        assert!(matches!(
            result,
            Err(SegmentUseCaseError::SessionRepository(SessionRepositoryError::SessionNotFound))
        ));
    }

    #[test]
    fn test_execute_with_missing_image_returns_error() {
        let image_jpg =
            read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Tumbler.jpg"))
                .unwrap();

        let loader = FileImageLoader::new();
        let image = loader.load(image_jpg).unwrap().into_image();
        let image_cache = SharedRenderedImageCacheInMemory::new();
        let session_repo = SessionRepositoryInMemory::new();
        let image_repo = OriginalImageRepositoryInMemory::new();
        let segmenter = Sam2Segmenter::new("models").expect("model load failed");
        let editing_session_repo = SAM2EditingSessionRepository::new();

        let session_id = SessionId::new();
        let session = Session::new(session_id, *image.image_id());
        session_repo.save(session);

        let usecase = SegmentUseCase::new(
            session_repo,
            image_repo,
            segmenter,
            editing_session_repo,
            image_cache,
        );

        let point = Point::new(Coordinate::new(30, 40), PointLabel::FOREGROUND);
        let input = SegmentInput::new(session_id, point);

        let result = usecase.execute(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_execute_with_missing_editing_session_returns_error() {
        let image_jpg =
            read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Tumbler.jpg"))
                .unwrap();

        let loader = FileImageLoader::new();
        let image = loader.load(image_jpg).unwrap().into_image();
        let image_cache = SharedRenderedImageCacheInMemory::new();
        let session_repo = SessionRepositoryInMemory::new();
        let image_repo = OriginalImageRepositoryInMemory::new();
        let editing_session_repo = SAM2EditingSessionRepository::new();
        let segmenter = Sam2Segmenter::new("models").expect("model load failed");

        let session_id = SessionId::new();
        let session = Session::new(session_id, *image.image_id());
        session_repo.save(session);
        image_repo.save(image);

        let usecase = SegmentUseCase::new(
            session_repo,
            image_repo,
            segmenter,
            editing_session_repo,
            image_cache,
        );

        let point = Point::new(Coordinate::new(30, 40), PointLabel::FOREGROUND);
        let input = SegmentInput::new(session_id, point);

        let result = usecase.execute(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_execute_saves_segmented_image_to_cache() {
        let image_jpg =
            read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Tumbler.jpg"))
                .unwrap();

        let (session_repo, image_repo, segmenter, editing_session_repo, image_cache, session_id) = _set_up(image_jpg);
        let usecase = SegmentUseCase::new(session_repo.clone(), image_repo.clone(), segmenter.clone(), editing_session_repo.clone(), image_cache.clone());

        let point = Point::new(Coordinate::new(30, 40), PointLabel::FOREGROUND);
        let input = SegmentInput::new(session_id, point);

        let output = usecase.execute(input).unwrap();
        let new_image_id = output.image_id();

        let cached = image_cache.take(new_image_id.clone());
        assert!(cached.is_ok());
        assert_eq!(cached.unwrap().image_id(), new_image_id);
    }

    #[ignore]
    #[test]
    fn test_visualize_segment_tumbler_usecase() {
        let image_jpg =
            read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Tumbler.jpg"))
                .unwrap();

        let (session_repo, image_repo, segmenter, editing_session_repo, image_cache, session_id) = _set_up(image_jpg);
        let usecase = SegmentUseCase::new(session_repo.clone(), image_repo.clone(), segmenter.clone(), editing_session_repo.clone(), image_cache.clone());

        let point1 = Point::new(Coordinate::new(572, 500), PointLabel::FOREGROUND);
        let input = SegmentInput::new(session_id, point1);

        let output = usecase.execute(input).unwrap();
        let new_image_id = output.image_id();
        let segmented = image_cache.take(new_image_id).unwrap();

        let (segmented_data, _, size) = segmented.into_data();
        let segmented_raw = segmented_data.into_image();

        let height= size.height();
        let width= size.width();

        let img = RgbaImage::from_raw(width as u32, height as u32, segmented_raw).unwrap();

        img.save("test_tumbler_silver.png").unwrap();
    }

    #[ignore]
    #[test]
    fn test_visualize_segment_my_hand_usecase() {
        let image_jpg =
            read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/My_hand.jpg"))
                .unwrap();

        let (session_repo, image_repo, segmenter, editing_session_repo, image_cache, session_id) = _set_up(image_jpg);
        let usecase = SegmentUseCase::new(session_repo.clone(), image_repo.clone(), segmenter.clone(), editing_session_repo.clone(), image_cache.clone());

        let point1 = Point::new(Coordinate::new(572, 500), PointLabel::FOREGROUND);
        let input = SegmentInput::new(session_id, point1);

        let output = usecase.execute(input).unwrap();
        let new_image_id = output.image_id();
        let segmented = image_cache.take(new_image_id).unwrap();

        let (segmented_data, _, size) = segmented.into_data();
        let segmented_raw = segmented_data.into_image();

        let height= size.height();
        let width= size.width();
        
        let img = RgbaImage::from_raw(width as u32, height as u32, segmented_raw).unwrap();

        img.save("test_my_hand.png").unwrap();
    }
}
