#[cfg(test)]
mod upload_usecase_test {
    use std::cell::RefCell;
    use std::fs;
    use std::rc::Rc;
    use crate::application::errors::application_errors::ApplicationErrors;
    use crate::application::errors::loading_errors::LoadingErrors;
    use crate::application::repository::image_repository::ImageRepository;
    use crate::application::usecase::upload_usecase::upload_input::UploadInput;
    use crate::application::usecase::upload_usecase::usecase::UploadUseCase;
    use crate::domain::repository::image_meta_repository::ImageMetaRepository;
    use crate::domain::repository::session_repository::SessionRepository;
    use crate::infrastructure::image_loader::FileImageLoader;
    use crate::infrastructure::repository::image_meta_repository::ImageMetaRepositoryInMemory;
    use crate::infrastructure::repository::image_repository::ImageRepositoryInMemory;
    use crate::infrastructure::repository::session_repository::SessionRepositoryInMemory;

    #[test]
    fn test_normal_execute() {
        let session_repo = Rc::new(RefCell::new(SessionRepositoryInMemory::new()));
        let image_repo = Rc::new(RefCell::new(ImageRepositoryInMemory::new()));
        let meta_repo = Rc::new(RefCell::new(ImageMetaRepositoryInMemory::new()));
        let loader = FileImageLoader::new();
        let mut usecase = UploadUseCase::new(
            Rc::clone(&session_repo),
            Rc::clone(&image_repo),
            Rc::clone(&meta_repo),
            loader);

        let image=fs::read("C:/Users/chiak/OneDrive/ドキュメント/ChickRen/Image_Crop1/src/test/test_image/Anti Cyclone.png").unwrap();
        let input = UploadInput::new(image);
        
        let output=usecase.execute(input).unwrap();
        let (session_id, image_id) = output.into_parts();

        assert!(session_repo.borrow().get(&session_id).is_some());
        assert!(meta_repo.borrow().get(&image_id).is_some());
        assert!(image_repo.borrow().get(&image_id).is_some());
    }

    #[test]
    fn test_invalid_input() {
        let session_repo = Rc::new(RefCell::new(SessionRepositoryInMemory::new()));
        let image_repo = Rc::new(RefCell::new(ImageRepositoryInMemory::new()));
        let meta_repo = Rc::new(RefCell::new(ImageMetaRepositoryInMemory::new()));
        let loader = FileImageLoader::new();
        let mut usecase = UploadUseCase::new(
            Rc::clone(&session_repo),
            Rc::clone(&image_repo),
            Rc::clone(&meta_repo),
            loader);

        let image=fs::read("C:/Users/chiak/OneDrive/ドキュメント/ChickRen/Image_Crop1/src/test/test_image/Unsupported.wav").unwrap();
        let input = UploadInput::new(image);

        let result=usecase.execute(input);

        assert_eq!(result, Err(ApplicationErrors::ImageLoadError(LoadingErrors::UnsupportedFormat)));
    }
}