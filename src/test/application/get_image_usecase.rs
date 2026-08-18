#[cfg(test)]
mod get_image_usecase_test {
    use std::fs;
    use std::path::Path;

    use crate::application::interface::image_loader::loader::ImageLoader;
    use crate::application::usecase::get_image_usecase::error::GetImageUseCaseError;
    use crate::application::usecase::get_image_usecase::get_image_input::GetImageInput;
    use crate::application::usecase::get_image_usecase::usecase::GetImageUseCase;
    use crate::application::usecase::upload_usecase::upload_input::UploadInput;
    use crate::application::usecase::upload_usecase::usecase::UploadUseCase;
    use crate::domain::value_object::image_id::image_id::ImageId;
    use crate::infrastructure::cache::shared_rendered_image_cache::SharedRenderedImageCacheInMemory;
    use crate::infrastructure::image_loader::FileImageLoader;
    use crate::infrastructure::repository::editing_session_repository::SAM2EditingSessionRepository;
    use crate::infrastructure::repository::image_repository::OriginalImageRepositoryInMemory;
    use crate::infrastructure::repository::session_repository::SessionRepositoryInMemory;
    use crate::infrastructure::segmenter::sam2::Sam2Segmenter;

    fn _set_up(image_jpg: Vec<u8>) -> (SharedRenderedImageCacheInMemory, ImageId) {
        let model_dir = "models";
        let loader = FileImageLoader::new();
        let session_repo = SessionRepositoryInMemory::new();
        let image_repo = OriginalImageRepositoryInMemory::new();
        let image_cache = SharedRenderedImageCacheInMemory::new();
        let segmenter = Sam2Segmenter::new(model_dir).unwrap();

        let editing_session_repo = SAM2EditingSessionRepository::new();

        let usecase = UploadUseCase::new(
            session_repo,
            image_repo,
            loader,
            segmenter,
            editing_session_repo,
            image_cache.clone(),
        );
        let input = UploadInput::new(image_jpg);
        let output = usecase.execute(input).unwrap();
        let (_, id) = output.into_session_id_and_image_id();

        (image_cache, id)
    }

    #[test]
    fn test_normal_execute() {
        let image_jpg = fs::read(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Normal_Image.png"),
        )
        .expect("image load failed");

        let (image_cache, id) = _set_up(image_jpg.clone());
        let usecase = GetImageUseCase::new(image_cache);

        let input = GetImageInput::new(id);
        let output = usecase.execute(input).unwrap();

        let loader = FileImageLoader::new();
        let loaded_image = loader.load(image_jpg).unwrap();
        let (original_data, _, _) = loaded_image.into_image().into_data();

        assert_eq!(output.into_image_data().0, original_data.into_image());
    }

    #[test]
    fn test_unknown_image_id_input() {
        let image_jpg = fs::read(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Normal_Image.png"),
        )
        .expect("image load failed");

        let (image_cache, id) = _set_up(image_jpg.clone());
        let usecase = GetImageUseCase::new(image_cache);

        let unknown_id = ImageId::new();
        let input = GetImageInput::new(unknown_id);

        let result = usecase.execute(input);

        assert!(matches!(result, Err(GetImageUseCaseError::ImageCache(_))))
    }
}
