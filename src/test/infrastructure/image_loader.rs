#[cfg(test)]
mod image_loader_test{

    use std::fs;
    use crate::{application::{errors::loading_errors::LoadingErrors, interface::image_loader::ImageLoader, usecase::upload_usecase::upload_input::UploadInput}, infrastructure::image_loader::FileImageLoader};
    
    #[test]
    fn test_normal_load(){
        let loader= FileImageLoader::new();

        let image=fs::read("C:/Users/chiak/OneDrive/ドキュメント/ChickRen/Image_Crop1/src/test/test_image/Anti Cyclone.png").unwrap();

        let result = loader.load(image);
        assert!(&result.is_ok());

        let (_image, size) = result.unwrap().into_image_and_size();
        assert_eq!(size.height(), 1080);
        assert_eq!(size.width(), 1920);
    }

    #[test]
    fn test_invalid_size(){
        let loader= FileImageLoader::new();

        let image=fs::read("C:/Users/chiak/OneDrive/ドキュメント/ChickRen/Image_Crop1/src/test/test_image/TooBigImage.png").unwrap();

        let result = loader.load(image);
        assert_eq!(result, Err(LoadingErrors::InvalidSize));
    }

    #[test]
    fn test_broken_image(){
        let loader= FileImageLoader::new();

        let image=fs::read("C:/Users/chiak/OneDrive/ドキュメント/ChickRen/Image_Crop1/src/test/test_image/BrokenImage.png").unwrap();

        let result = loader.load(image);
        assert_eq!(result, Err(LoadingErrors::CorruptedImage))
    }

    #[test]
    fn test_unsupported_extension(){
        let loader= FileImageLoader::new();

        let image=fs::read("C:/Users/chiak/OneDrive/ドキュメント/ChickRen/Image_Crop1/src/test/test_image/Unsupported.wav").unwrap();

        let result = loader.load(image);
        assert_eq!(result, Err(LoadingErrors::UnsupportedFormat));
    }
}