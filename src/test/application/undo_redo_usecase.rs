#[cfg(test)]
mod undo_redo_usecase_test {
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
            usecase::{
                redo_usecase::{redo_input::RedoInput, usecase::RedoUseCase},
                undo_usecase::{undo_input::UndoInput, usecase::UndoUseCase},
            },
        },
        domain::{
            entity::{image::Image, session::Session},
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
                shared_editing_session_repository::SharedEditingSessionRepository,
                shared_image_repository::SharedImageRepository,
                shared_session_repository::SharedSessionRepository,
            },
            segmenter::{
                sam2_data::{SAM2InferenceContext, SAM2StaticContext},
                shared_sam2::SharedSAM2Segmenter,
            },
        },
    };

    fn _set_up() -> (
        SharedSessionRepository,
        SharedImageRepository,
        SharedSAM2Segmenter,
        SharedEditingSessionRepository,
        CommonEditingSession<SAM2StaticContext, SAM2InferenceContext>,
        Image,
        SessionId,
    ) {
        let model_dir = "models";
        let loader = FileImageLoader::new();
        let mut session_repo = SharedSessionRepository::new();
        let image_repo = SharedImageRepository::new();
        let mut segmenter = SharedSAM2Segmenter::new(model_dir).unwrap();
        let editing_session_repo = SharedEditingSessionRepository::new();
        let history = PointHistory::new(40);

        let image_jpg =
            fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Tumbler.jpg"))
                .unwrap();

        let loaded_image = loader.load(image_jpg).unwrap();
        let original_image = loaded_image.into_image();
        let original_image_id = original_image.image_id().clone();

        let original_session_id = SessionId::new();
        let original_session = Session::new(original_session_id, original_image_id.clone());

        let static_context = segmenter.prepare_static_context(&original_image).unwrap();
        let inference_context = segmenter
            .prepare_inference_context(&original_image)
            .unwrap();

        let editing_session = CommonEditingSession::new(history, static_context, inference_context);

        session_repo.save(original_session);

        (
            session_repo,
            image_repo,
            segmenter,
            editing_session_repo,
            editing_session,
            original_image,
            original_session_id,
        )
    }

    #[test]
    fn test_normal_undo_execute() {
        let (
            session_repo,
            mut image_repo,
            mut segmenter,
            mut editing_session_repo,
            mut editing_session,
            original_image,
            original_session_id,
        ) = _set_up();

        let point = Point::new(Coordinate::new(50, 100), PointLabel::FOREGROUND);
        editing_session.add_point(point);

        let (first_context, first_segment) = segmenter
            .segment(
                &original_image,
                editing_session.static_context(),
                editing_session.inference_context(),
                editing_session.points(),
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

    #[test]
    fn test_normal_redo_execute() {
        let (
            session_repo,
            mut image_repo,
            mut segmenter,
            mut editing_session_repo,
            mut editing_session,
            original_image,
            original_session_id,
        ) = _set_up();

        let point = Point::new(Coordinate::new(50, 100), PointLabel::FOREGROUND);
        editing_session.add_point(point);

        let (first_context, first_segment) = segmenter
            .segment(
                &original_image,
                editing_session.static_context(),
                editing_session.inference_context(),
                editing_session.points(),
            )
            .unwrap();

        editing_session.set_inference_context(first_context);
        editing_session_repo.save(&original_session_id, editing_session);

        let mut editing_session = editing_session_repo.get(&original_session_id).unwrap();

        let point = Point::new(Coordinate::new(60, 140), PointLabel::FOREGROUND);
        editing_session.add_point(point);

        let (second_context, second_segment) = segmenter
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

        let mut undo_usecase = UndoUseCase::new(
            session_repo.clone(),
            image_repo.clone(),
            segmenter.clone(),
            editing_session_repo.clone(),
        );
        let input = UndoInput::new(original_session_id);

        let output = undo_usecase.execute(input).unwrap();

        let (output_session_id, image_id) = output.into_session_id_and_image_id();
        assert_eq!(output_session_id, original_session_id);

        let output_image = image_repo.get(&image_id, ImageKind::Segmented).unwrap();

        let output_data = output_image.image_data();
        let (first_segment_data, _) = first_segment.into_image_and_size();

        assert_eq!(output_data, &first_segment_data);

        let mut redo_usecase = RedoUseCase::new(
            session_repo.clone(),
            image_repo.clone(),
            segmenter.clone(),
            editing_session_repo.clone(),
        );

        let redo_input = RedoInput::new(original_session_id);
        let output = redo_usecase.execute(redo_input).unwrap();

        let (output_session_id, output_image_id) = output.into_session_id_and_image_id();
        let output_image = image_repo
            .get(&output_image_id, ImageKind::Segmented)
            .unwrap();
        assert_eq!(output_session_id, original_session_id);

        let output_data = output_image.image_data();
        let (second_segment_data, _) = second_segment.into_image_and_size();

        assert_eq!(output_data, &second_segment_data);
    }
}
