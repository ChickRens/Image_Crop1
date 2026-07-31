use crate::application::{interface::image_loader::error::LoadingErrors, types::loaded_image::LoadedImage};

pub trait ImageLoader {
    fn load(&self, data: Vec<u8>) -> Result<LoadedImage, LoadingErrors>;
}
