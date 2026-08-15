#[cfg(test)]
mod image_loader_test {
    use crate::{
        application::interface::image_loader::{error::LoadingError, loader::ImageLoader},
        infrastructure::image_loader::FileImageLoader,
    };
    use std::{fs::read, path::Path};

    #[test]
    fn test_normal_load() {
        let loader = FileImageLoader::new();

        let image = read(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Normal_Image.png"),
        )
        .unwrap();

        let result = loader.load(image);
        assert!(result.is_ok());

        let image = result.unwrap().into_image();
        let size = image.image_size();
        assert_eq!(size.height(), 500);
        assert_eq!(size.width(), 500);
    }

    #[test]
    fn test_invalid_size() {
        let loader = FileImageLoader::new();

        let image =
            read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Huge_Image.png"))
                .unwrap();

        let result = loader.load(image);
        assert!(matches!(result, Err(LoadingError::ImageSize(_))));
    }

    #[test]
    fn test_broken_image() {
        let loader = FileImageLoader::new();

        let image = read(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Broken_Image.png"),
        )
        .unwrap();

        let result = loader.load(image);
        assert!(matches!(result, Err(LoadingError::CorruptedImage(_))))
    }

    #[test]
    fn test_unsupported_extension() {
        let loader = FileImageLoader::new();

        let image =
            read(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test/test_image/Unsupported.wav"))
                .unwrap();

        let result = loader.load(image);
        assert!(matches!(result, Err(LoadingError::UnsupportedFormat(_))));
    }
}
