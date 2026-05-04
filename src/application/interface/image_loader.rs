use crate::application::errors::loading_errors::LoadingErrors;
use crate::application::types::loaded_image::LoadedImage;

pub trait ImageLoader {
    fn load(&self, data: Vec<u8>) -> Result<LoadedImage, LoadingErrors>;
}
