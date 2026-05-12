use crate::application::errors::loading_errors::LoadingErrors;
use crate::application::interface::image_loader::ImageLoader;
use crate::application::types::loaded_image::LoadedImage;
use crate::domain::entity::image::Image;
use crate::domain::value_object::image_data::ImageData;
use crate::domain::value_object::image_id::ImageId;
use crate::domain::value_object::image_size::ImageSize;
use image::DynamicImage;
use image::ImageBuffer;
use image::ImageError;
use image::Rgb;

pub struct FileImageLoader;

impl ImageLoader for FileImageLoader {
    fn load(&self, data: Vec<u8>) -> Result<LoadedImage, LoadingErrors> {
        let img: DynamicImage = image::load_from_memory(&data).map_err(|err| match err {
            ImageError::Unsupported(_) => LoadingErrors::UnsupportedFormat,
            ImageError::Decoding(_) => LoadingErrors::CorruptedImage,
            _ => LoadingErrors::CorruptedImage,
        })?;

        let width: u32 = img.width();
        let height: u32 = img.height();

        let image_size: ImageSize =
            ImageSize::new(height as u16, width as u16).map_err(|_| LoadingErrors::InvalidSize)?;

        let rgb_image: ImageBuffer<Rgb<u8>, Vec<u8>> = img.to_rgb8();
        let raw_pixels: Vec<u8> = rgb_image.into_raw();

        let image_data = ImageData::new(raw_pixels);
        let image_id = ImageId::new();
        let image = Image::new(image_data, image_id, image_size, 10);

        Ok(LoadedImage::new(image))
    }
}

impl FileImageLoader {
    pub fn new() -> Self {
        Self
    }
}
