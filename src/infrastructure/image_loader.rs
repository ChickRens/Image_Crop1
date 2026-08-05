use crate::application::interface::image_loader::error::LoadingError;
use crate::application::interface::image_loader::loader::ImageLoader;
use crate::application::types::loaded_image::LoadedImage;
use crate::domain::entity::image::image::Image;
use crate::domain::value_object::image_data::ImageData;
use crate::domain::value_object::image_id::image_id::ImageId;
use crate::domain::value_object::image_size::image_size::ImageSize;
use image::DynamicImage;
use image::ImageError;

pub struct FileImageLoader;

impl ImageLoader for FileImageLoader {
    fn load(&self, data: Vec<u8>) -> Result<LoadedImage, LoadingError> {
        let img: DynamicImage = image::load_from_memory(&data).map_err(|err| match err {
            ImageError::Unsupported(_) => LoadingError::UnsupportedFormat,
            ImageError::Decoding(_) => LoadingError::CorruptedImage,
            _ => LoadingError::CorruptedImage,
        })?;

        let width: u32 = img.width();
        let height: u32 = img.height();

        let image_size = ImageSize::new(height as u16, width as u16)?;

        let rgb_image = img.to_rgb8();
        let raw_pixels = rgb_image.into_raw();

        let image_data = ImageData::new(raw_pixels);
        let image_id = ImageId::new();
        let image = Image::new(image_data, image_id, image_size);

        Ok(LoadedImage::new(image))
    }
}

impl FileImageLoader {
    pub fn new() -> Self {
        Self
    }
}
