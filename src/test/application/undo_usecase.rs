#[cfg(test)]
mod undo_usecase_test {
    use std::{fs, path::Path};

    use crate::{
        application::{
            interface::{
                editing_session_repository::EditingSessionRepository, image_loader::ImageLoader,
                image_segmenter::ImageSegmenter,
            },
            types::{
                editing_session::{CommonEditingSession, EditingSession},
                point_history::PointHistory,
            },
            usecase::undo_usecase::{undo_input::UndoInput, usecase::UndoUseCase},
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
                shared_image_repository::SharedImageRepository,
                shared_session_repository::SharedSessionRepository,
            },
            segmenter::shared_sam2::SharedSAM2Segmenter,
        },
    };

    #[test]
    fn test_normal_execute() {
        let model_dir = "models";
        let loader = FileImageLoader::new();
        let mut session_repo = SharedSessionRepository::new();
        let mut image_repo = SharedImageRepository::new();
        let mut segmenter = SharedSAM2Segmenter::new(model_dir).unwrap();
        let mut editing_session_repo = SAM2EditingSessionRepository::new();
        let history = PointHistory::new(40);

        let image_jpg =
            fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Tumbler.jpg"))
                .unwrap();

        let loaded_image = loader.load(image_jpg).unwrap();
        let original_image = loaded_image.into_image();
        let original_image_id = original_image.image_id();

        let original_session_id = SessionId::new();
        let original_session = Session::new(original_session_id, original_image_id.clone());

        let static_context = segmenter.prepare_static_context(&original_image).unwrap();
        let inference_context = segmenter
            .prepare_inference_context(&original_image)
            .unwrap();

        let mut editing_session =
            CommonEditingSession::new(history, static_context, inference_context);

        let point = Point::new(Coordinate::new(50, 100), PointLabel::FOREGROUND);
        editing_session.add_point(point);

        let (first_context, first_segment) = segmenter
            .segment(
                &original_image,
                editing_session.static_context(),
                editing_session.inference_context(),
                editing_session.points()
            )
            .unwrap();

        editing_session.set_inference_context(first_context);
        editing_session_repo.save(&original_session_id, editing_session);

        let mut editing_session = editing_session_repo.get(&original_session_id).unwrap();

        let point = Point::new(Coordinate::new(60, 140), PointLabel::FOREGROUND);
        editing_session.add_point(point);
            
        let (second_context, _second_segment) = segmenter
            .segment(
                &original_image,
                editing_session.static_context(),
                editing_session.inference_context(),
                editing_session.points(),
            )
            .unwrap();

        editing_session.set_inference_context(second_context);
        editing_session_repo.save(&original_session_id, editing_session);

        image_repo.save(original_image.clone(), ImageKind::Original);
        session_repo.save(original_session);

        let mut usecase = UndoUseCase::new(
            session_repo,
            image_repo.clone(),
            segmenter,
            editing_session_repo,
        );
        let input = UndoInput::new(original_session_id);

        let output = usecase.execute(input).unwrap();

        let (output_session_id, image_id) = output.into_session_id_and_image_id();
        assert_eq!(output_session_id, original_session_id);

        let output_image = image_repo.get(&image_id, ImageKind::Segmented).unwrap();

        let output_data = output_image.image_data();
        let (first_segment_data, _) = first_segment.into_image_and_size();

        assert_eq!(output_data, &first_segment_data)
    }
}
