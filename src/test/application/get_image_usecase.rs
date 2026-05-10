#[cfg(test)]
mod get_image_usecase_test {
    use std::fs;
    use std::path::Path;

    use crate::application::interface::image_loader::ImageLoader;
    use crate::application::usecase::get_image_usecase::get_image_input::GetImageInput;
    use crate::application::usecase::get_image_usecase::usecase::GetImageUseCase;
    use crate::domain::entity::session::Session;
    use crate::domain::repository::image_repository::ImageRepository;
    use crate::domain::repository::session_repository::SessionRepository;
    use crate::domain::value_object::{image_kind::ImageKind, session_id::SessionId};
    use crate::infrastructure::image_loader::FileImageLoader;
    use crate::infrastructure::repository::image_repository::ImageRepositoryInMemory;
    use crate::infrastructure::repository::session_repository::SessionRepositoryInMemory;

    #[test]
    fn test_normal_execute() {
        let loader = FileImageLoader::new();
        let mut image_repo = ImageRepositoryInMemory::new();
        let mut session_repo = SessionRepositoryInMemory::new();

        let session_id = SessionId::new();

        let image_bytes = fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Normal_Image.png")).unwrap();
        let loaded_image = loader.load(image_bytes).unwrap();
        let image = loaded_image.into_image();
        let image_id = image.image_id().clone();

        image_repo.save(image.clone(), ImageKind::Original);

        let session = Session::new(session_id.clone(), image_id.clone());
        session_repo.save(session);

        let mut usecase = GetImageUseCase::new(session_repo, image_repo);

        let input = GetImageInput::new(session_id, image_id, ImageKind::Original);
        let result = usecase.execute(input);

        assert!(result.is_ok());
    }
}
