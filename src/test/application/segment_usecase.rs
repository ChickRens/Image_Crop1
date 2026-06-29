#[cfg(test)]
mod segment_usecase_test {
    use image::RgbaImage;

    use crate::{
        application::{
            errors::{application_errors::ApplicationErrors, loading_errors::LoadingErrors},
            interface::{
                editing_session_repository::EditingSessionRepository, image_loader::ImageLoader,
                image_segmenter::ImageSegmenter,
            },
            types::{editing_session::CommonEditingSession, point_history::PointHistory},
            usecase::{
                segment_usecase::{segment_input::SegmentInput, usecase::SegmentUseCase},
                upload_usecase::{upload_input::UploadInput, usecase::UploadUseCase},
            },
        },
        domain::{
            entity::session::Session,
            repository::{
                image_repository::ImageRepository, session_repository::SessionRepository,
            },
            value_object::{
                coordinate::Coordinate,
                image_kind::ImageKind,
                point::{Point, PointLabel},
                session_id::SessionId,
            },
        },
        infrastructure::{
            image_loader::FileImageLoader,
            repository::{
                editing_session_repository::SAM2EditingSessionRepository,
                image_repository::ImageRepositoryInMemory,
                session_repository::SessionRepositoryInMemory,
                shared_image_repository::SharedImageRepository,
                shared_session_repository::SharedSessionRepository,
            },
            segmenter::sam2::Sam2Segmenter,
        },
    };

    use std::fs;
    use std::path::Path;

    #[test]
    fn test_normal_execute() {
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

        let original_session_id = SessionId::new();
        let point = Point::new(Coordinate::new(500, 500), PointLabel::FOREGROUND);
        let input = SegmentInput::new(original_session_id, *image.image_id(), point);

        let session = Session::new(original_session_id, *image.image_id());
        let static_context = segmenter.prepare_static_context(&image).unwrap();
        let inference_context = segmenter.prepare_inference_context(&image).unwrap();

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
        let input = SegmentInput::new(original_session_id, *image.image_id(), point1);

        let session = Session::new(original_session_id, *image.image_id());
        let inference_context = segmenter.prepare_inference_context(&image).unwrap();
        let static_context = segmenter.prepare_static_context(&image).unwrap();
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
        let input = SegmentInput::new(original_session_id, *image.image_id(), point1);

        let session = Session::new(original_session_id, *image.image_id());

        let static_context = segmenter.prepare_static_context(&image).unwrap();
        let inference_context = segmenter.prepare_inference_context(&image).unwrap();

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
