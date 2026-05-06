#[cfg(test)]
mod get_image_usecase_test{
    use std::fs;

    use crate::application::interface::image_loader::ImageLoader;
    use crate::application::repository::image_repository::ImageRepository;
    use crate::application::usecase::config::MAX_HISTORY;
    use crate::application::usecase::get_image_usecase::get_image_input::GetImageInput;
    use crate::application::usecase::get_image_usecase::usecase::GetImageUseCase;
    use crate::domain::entity::image_meta::ImageMeta;
    use crate::domain::entity::session::Session;
    use crate::domain::repository::image_meta_repository::ImageMetaRepository;
    use crate::domain::repository::session_repository::SessionRepository;
    use crate::domain::value_object::image_id::ImageId;
    use crate::domain::value_object::session_id::SessionId;
    use crate::infrastructure::image_loader::FileImageLoader;
    use crate::infrastructure::repository::image_meta_repository::ImageMetaRepositoryInMemory;
    use crate::infrastructure::repository::image_repository::ImageRepositoryInMemory;
    use crate::infrastructure::repository::session_repository::SessionRepositoryInMemory;

    #[test]
    fn test_normal_execute(){
        let loader = FileImageLoader::new();
        let mut meta_repo = ImageMetaRepositoryInMemory::new();
        let mut image_repo = ImageRepositoryInMemory::new();
        let mut session_repo = SessionRepositoryInMemory::new();
        
        let image_id = ImageId::new();
        let session_id = SessionId::new();
        let max_history = MAX_HISTORY;

        let image_bytes = fs::read("C:/Users/chiak/OneDrive/ドキュメント/ChickRen/Image_Crop1/src/test/test_image/Spot Late_4.png").unwrap();
        let loaded_image=loader.load(image_bytes).unwrap();
        let (image, size) = loaded_image.into_image_and_size();

        image_repo.save(image, image_id);
        
        let meta= ImageMeta::new(image_id, size, max_history);
        meta_repo.save(meta);

        let session = Session::new(session_id, image_id);
        session_repo.save(session);
        
        let mut usecase = GetImageUseCase::new(session_repo, image_repo, meta_repo);
        
        let input=GetImageInput::new(session_id, image_id);
        let result=usecase.execute(input);

        assert!(result.is_ok())
    }


}
