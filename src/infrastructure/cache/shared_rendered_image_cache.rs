use std::sync::Arc;

use crate::{application::{interface::rendered_image_cache::{cache::RenderedImageCache, error::RenderedCacheError}, types::rendered_image::RenderedImage}, domain::value_object::image_id::image_id::ImageId, infrastructure::cache::rendered_image_cache::RenderedImageCacheInMemory};

#[derive(Clone)]
pub struct SharedRenderedImageCacheInMemory {
    caches: Arc<RenderedImageCacheInMemory>
}

impl RenderedImageCache for SharedRenderedImageCacheInMemory {
    fn save(&self, image: RenderedImage) {
        self.caches.save(image);
    }

    fn take(&self, id: ImageId) -> Result<RenderedImage, RenderedCacheError> {
        self.caches.take(id)
    }
}

impl SharedRenderedImageCacheInMemory {
    pub fn new() -> Self {
        Self { caches: Arc::new(RenderedImageCacheInMemory::new()) }
    }
}