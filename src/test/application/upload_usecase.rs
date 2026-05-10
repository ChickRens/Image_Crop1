#[cfg(test)]
mod upload_usecase_test {
    use std::fs;
    use std::path::Path;
    use crate::application::errors::application_errors::ApplicationErrors;
    use crate::application::errors::loading_errors::LoadingErrors;
    use crate::application::usecase::upload_usecase::upload_input::UploadInput;
    use crate::application::usecase::upload_usecase::usecase::UploadUseCase;
    use crate::infrastructure::image_loader::FileImageLoader;
    use crate::infrastructure::repository::image_repository::ImageRepositoryInMemory;
    use crate::infrastructure::repository::session_repository::SessionRepositoryInMemory;

    #[test]
    fn test_normal_execute() {
        let session_repo = SessionRepositoryInMemory::new();
        let image_repo = ImageRepositoryInMemory::new();
        let loader = FileImageLoader::new();
        let mut usecase = UploadUseCase::new(session_repo, image_repo, loader);

        let image = fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Anti Cyclone.png")).unwrap();
        let input = UploadInput::new(image);
        let output = usecase.execute(input).unwrap();
        let (session_id, _image_id) = output.into_session_id_and_image_id();

        assert!(!session_id.value().to_string().is_empty());
    }

    #[test]
    fn test_invalid_input() {
        let session_repo = SessionRepositoryInMemory::new();
        let image_repo = ImageRepositoryInMemory::new();
        let loader = FileImageLoader::new();
        let mut usecase = UploadUseCase::new(session_repo, image_repo, loader);

        let image = fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Unsupported.wav")).unwrap();
        let input = UploadInput::new(image);

        let result = usecase.execute(input);

        assert_eq!(result, Err(ApplicationErrors::ImageLoadError(LoadingErrors::UnsupportedFormat)));
    }
}