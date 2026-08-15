#[cfg(test)]
mod upload_usecase_test {
    use crate::{
        application::{
            interface::{
                editing_session_repository::repository::EditingSessionRepository,
                image_loader::error::LoadingError, rendered_image_cache::cache::RenderedImageCache,
            },
            usecase::upload_usecase::{
                error::UploadUseCaseError, upload_input::UploadInput, usecase::UploadUseCase,
            },
        },
        domain::repository::{
            original_image_repository::repository::OriginalImageRepository,
            session_repository::repository::SessionRepository,
        },
        infrastructure::{
            cache::{
                rendered_image_cache::RenderedImageCacheInMemory,
                shared_rendered_image_cache::SharedRenderedImageCacheInMemory,
            },
            image_loader::FileImageLoader,
            repository::{
                editing_session_repository::SAM2EditingSessionRepository,
                image_repository::OriginalImageRepositoryInMemory,
                session_repository::SessionRepositoryInMemory,
                shared_editing_session_repository::SharedEditingSessionRepository,
                shared_image_repository::SharedOriginalImageRepository,
                shared_session_repository::SharedSessionRepository,
            },
            segmenter::sam2::Sam2Segmenter,
        },
    };
    use std::{fs::read, path::Path};

    fn _set_up() -> (
        SharedSessionRepository,
        SharedOriginalImageRepository,
        FileImageLoader,
        Sam2Segmenter,
        SharedEditingSessionRepository,
        SharedRenderedImageCacheInMemory,
    ) {
        let model_dir = "models";
        let session_repo = SharedSessionRepository::new();
        let image_repo = SharedOriginalImageRepository::new();
        let loader = FileImageLoader::new();
        let segmenter = Sam2Segmenter::new(model_dir).unwrap();
        let editing_session_repo = SharedEditingSessionRepository::new();
        let image_cache = SharedRenderedImageCacheInMemory::new();

        (
            session_repo,
            image_repo,
            loader,
            segmenter,
            editing_session_repo,
            image_cache,
        )
    }

    #[test]
    fn test_normal_execute() {
        let image_jpg = read(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Normal_Image.png"),
        )
        .unwrap();

        let (session_repo, image_repo, loader, segmenter, editing_session_repo, image_cache) =
            _set_up();
        let usecase = UploadUseCase::new(
            session_repo.clone(),
            image_repo.clone(),
            loader,
            segmenter,
            editing_session_repo.clone(),
            image_cache.clone(),
        );

        let input = UploadInput::new(image_jpg);
        let output = usecase.execute(input).unwrap();
        let (session_id, image_id) = output.into_session_id_and_image_id();

        let image_repo_result = image_repo.get(&image_id);
        let session_repo_result = session_repo.get(&session_id);
        let editing_session_repo_result = editing_session_repo.get(&session_id);
        let image_cache_result = image_cache.take(image_id);

        assert!(image_repo_result.is_ok());
        assert!(session_repo_result.is_ok());
        assert!(editing_session_repo_result.is_ok());
        assert!(image_cache_result.is_ok());

        let image = image_repo_result.unwrap();
        let cache = image_cache_result.unwrap();

        let (image_data, image_id, image_size) = image.into_data();
        let (cache_data, cache_id, cache_size) = cache.into_data();

        assert_eq!(image_data, cache_data);
        assert_eq!(image_id, cache_id);
        assert_eq!(image_size, cache_size);
    }

    #[test]
    fn test_invalid_input() {
        let image =
            read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Unsupported.wav"))
                .unwrap();

        let (session_repo, image_repo, loader, segmenter, editing_session_repo, image_cache) =
            _set_up();
        let usecase = UploadUseCase::new(
            session_repo.clone(),
            image_repo.clone(),
            loader,
            segmenter,
            editing_session_repo.clone(),
            image_cache.clone(),
        );

        let input = UploadInput::new(image);

        let result = usecase.execute(input);

        assert!(matches!(
            result,
            Err(UploadUseCaseError::Loader(LoadingError::UnsupportedFormat(
                _
            )))
        ));
    }
}
