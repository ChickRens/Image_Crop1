use crate::{application::{interface::rendered_image_cache::error::RenderedCacheError, types::rendered_image::RenderedImage}, domain::value_object::image_id::image_id::ImageId};

pub trait RenderedImageCache {
    fn save(&self, image: RenderedImage);
    fn take(&self, id: ImageId) -> Result<RenderedImage, RenderedCacheError>; 
}