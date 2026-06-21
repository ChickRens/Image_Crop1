#[cfg(test)]
mod upload_usecase_test {
    use crate::application::errors::application_errors::ApplicationErrors;
    use crate::application::errors::loading_errors::LoadingErrors;
    use crate::application::usecase::upload_usecase::upload_input::UploadInput;
    use crate::application::usecase::upload_usecase::usecase::UploadUseCase;
    use crate::infrastructure::image_loader::FileImageLoader;
    use crate::infrastructure::repository::editing_session_repository::SAM2EditingSessionRepository;
    use crate::infrastructure::repository::image_repository::ImageRepositoryInMemory;
    use crate::infrastructure::repository::session_repository::SessionRepositoryInMemory;
    use crate::infrastructure::segmenter::sam2::Sam2Segmenter;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_normal_execute() {
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
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Normal_Image.png"),
        )
        .unwrap();
        let input = UploadInput::new(image);
        let output = usecase.execute(input).unwrap();
        let (session_id, _image_id) = output.into_session_id_and_image_id();

        assert!(!session_id.value().to_string().is_empty());
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
