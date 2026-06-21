#[cfg(test)]
mod segment_usecase_test {
    use image::RgbaImage;

    use crate::application::errors::application_errors::ApplicationErrors;
    use crate::application::errors::loading_errors::LoadingErrors;
    use crate::application::interface::editing_session_repository::EditingSessionRepository;
    use crate::application::interface::image_loader::ImageLoader;
    use crate::application::interface::image_segmenter::ImageSegmenterPreparing;
    use crate::application::types::editing_session::CommonEditingSession;
    use crate::application::types::point_history::PointHistory;
    use crate::application::usecase::segment_usecase::segment_input::SegmentInput;
    use crate::application::usecase::segment_usecase::usecase::SegmentUseCase;
    use crate::application::usecase::upload_usecase::upload_input::UploadInput;
    use crate::application::usecase::upload_usecase::usecase::UploadUseCase;
    use crate::domain::entity::session::Session;
    use crate::domain::repository::image_repository::ImageRepository;
    use crate::domain::repository::session_repository::SessionRepository;
    use crate::domain::value_object::coordinate::Coordinate;
    use crate::domain::value_object::image_kind::ImageKind;
    use crate::domain::value_object::point::{Point, PointLabel};
    use crate::domain::value_object::session_id::SessionId;
    use crate::infrastructure::image_loader::FileImageLoader;
    use crate::infrastructure::repository::editing_session_repository::SAM2EditingSessionRepository;
    use crate::infrastructure::repository::image_repository::ImageRepositoryInMemory;
    use crate::infrastructure::repository::session_repository::SessionRepositoryInMemory;
    use crate::infrastructure::repository::shared_image_repository::SharedImageRepository;
    use crate::infrastructure::repository::shared_session_repository::SharedSessionRepository;
    use crate::infrastructure::segmenter::sam2::Sam2Segmenter;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_normal_execute() {
        let model_dir = "models";
        let mut session_repo = SessionRepositoryInMemory::new();
        let mut image_repo = ImageRepositoryInMemory::new();
        let loader = FileImageLoader::new();
        let mut segmenter = Sam2Segmenter::new(model_dir).unwrap();
        let mut editing_session_repo = SAM2EditingSessionRepository::new();
        let history = PointHistory::new(40);

        let image_jpg =
            fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Tumbler.jpg"))
                .unwrap();

        let loaded_image = loader.load(image_jpg).unwrap();
        let image = loaded_image.into_image();

        let original_session_id = SessionId::new();
        let point = Point::new(Coordinate::new(500, 500), PointLabel::FOREGROUND);
        let input = SegmentInput::new(original_session_id, *image.image_id(), [point].to_vec());

        let session = Session::new(original_session_id, *image.image_id());
        let (static_context, inference_context) = segmenter.prepare(&image).unwrap();
        let editing_session = CommonEditingSession::new(history, static_context, inference_context);

        session_repo.save(session);
        editing_session_repo.save(&original_session_id, editing_session);
        image_repo.save(image, ImageKind::Original);

        let mut usecase =
            SegmentUseCase::new(session_repo, image_repo, segmenter, editing_session_repo);

        let output = usecase.execute(input).unwrap();
        let (session_id, image_id) = output.into_session_id_and_image_id();
        assert!(!image_id.value().to_string().is_empty());
        assert_eq!(session_id, original_session_id);
    }

    #[ignore]
    #[test]
    fn test_visualize_segment_tumbler_usecase() {
        let model_dir = "models";
        let mut session_repo = SharedSessionRepository::new();
        let mut image_repo = SharedImageRepository::new();
        let loader = FileImageLoader::new();
        let mut segmenter = Sam2Segmenter::new(model_dir).unwrap();
        let mut editing_session_repo = SAM2EditingSessionRepository::new();
        let history = PointHistory::new(40);

        let image_jpg =
            fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Tumbler.jpg"))
                .unwrap();

        let loaded_image = loader.load(image_jpg).unwrap();
        let image = loaded_image.into_image();
        let (height, width) = (image.image_size().height(), image.image_size().width());

        let original_session_id = SessionId::new();
        let point1 = Point::new(Coordinate::new(572, 500), PointLabel::FOREGROUND);
        let point2 = Point::new(Coordinate::new(925, 402), PointLabel::FOREGROUND);
        let point3 = Point::new(Coordinate::new(500, 760), PointLabel::BACKGROUND);
        let input = SegmentInput::new(
            original_session_id,
            *image.image_id(),
            [point1, point2, point3].to_vec(),
        );

        let session = Session::new(original_session_id, *image.image_id());
        let (static_context, inference_context) = segmenter.prepare(&image).unwrap();
        let editing_session = CommonEditingSession::new(history, static_context, inference_context);

        session_repo.save(session);
        editing_session_repo.save(&original_session_id, editing_session);
        image_repo.save(image, ImageKind::Original);

        let mut usecase = SegmentUseCase::new(
            session_repo,
            image_repo.clone(),
            segmenter,
            editing_session_repo,
        );

        let output = usecase.execute(input).unwrap();
        let (session_id, image_id) = output.into_session_id_and_image_id();
        let segmented = image_repo.get(&image_id, ImageKind::Segmented).unwrap();
        let (segmented_data, _, _) = segmented.into_data();
        let segmented_raw = segmented_data.into_image();
        assert!(!image_id.value().to_string().is_empty());
        assert_eq!(session_id, original_session_id);

        let img = RgbaImage::from_raw(width as u32, height as u32, segmented_raw).unwrap();

        img.save("test_tumbler_silver.png").unwrap();
    }

    #[ignore]
    #[test]
    fn test_visualize_segment_my_hand_usecase() {
        let model_dir = "models";
        let mut session_repo = SharedSessionRepository::new();
        let mut image_repo = SharedImageRepository::new();
        let loader = FileImageLoader::new();
        let mut segmenter = Sam2Segmenter::new(model_dir).unwrap();
        let mut editing_session_repo = SAM2EditingSessionRepository::new();
        let history = PointHistory::new(40);

        let image_jpg =
            fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/My_hand.jpg"))
                .unwrap();

        let loaded_image = loader.load(image_jpg).unwrap();
        let image = loaded_image.into_image();
        let (height, width) = (image.image_size().height(), image.image_size().width());

        let original_session_id = SessionId::new();
        let point1 = Point::new(Coordinate::new(697, 851), PointLabel::FOREGROUND);
        let point2 = Point::new(Coordinate::new(100, 1000), PointLabel::BACKGROUND);
        let point3 = Point::new(Coordinate::new(400, 512), PointLabel::FOREGROUND);
        let input = SegmentInput::new(
            original_session_id,
            *image.image_id(),
            [point1, point2, point3].to_vec(),
        );

        let session = Session::new(original_session_id, *image.image_id());
        let (static_context, inference_context) = segmenter.prepare(&image).unwrap();
        let editing_session = CommonEditingSession::new(history, static_context, inference_context);

        session_repo.save(session);
        editing_session_repo.save(&original_session_id, editing_session);
        image_repo.save(image, ImageKind::Original);

        let mut usecase = SegmentUseCase::new(
            session_repo,
            image_repo.clone(),
            segmenter,
            editing_session_repo,
        );

        let output = usecase.execute(input).unwrap();
        let (session_id, image_id) = output.into_session_id_and_image_id();
        let segmented = image_repo.get(&image_id, ImageKind::Segmented).unwrap();
        let (segmented_data, _, _) = segmented.into_data();
        let segmented_raw = segmented_data.into_image();
        assert!(!image_id.value().to_string().is_empty());
        assert_eq!(session_id, original_session_id);

        let img = RgbaImage::from_raw(width as u32, height as u32, segmented_raw).unwrap();

        img.save("test_my_hand.png").unwrap();
    }

    #[test]
    fn test_invalid_input() {
        let model_dir = "models";
        let session_repo = SessionRepositoryInMemory::new();
        let image_repo = ImageRepositoryInMemory::new();
        let loader = FileImageLoader::new();
        let segmenter = Sam2Segmenter::new(model_dir).unwrap();
        let editing_session_repo = SAM2EditingSessionRepository::new();

        let mut usecase = UploadUseCase::new(
            session_repo,
            image_repo,
            loader,
            segmenter,
            editing_session_repo,
        );

        let image = fs::read(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Unsupported.wav"),
        )
        .unwrap();
        let input = UploadInput::new(image);

        let result = usecase.execute(input);

        assert_eq!(
            result,
            Err(ApplicationErrors::ImageLoadError(
                LoadingErrors::UnsupportedFormat
            ))
        );
    }
}
