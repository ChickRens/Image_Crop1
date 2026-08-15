use std::{collections::HashMap, sync::Mutex};

use crate::{
    application::{
        interface::rendered_image_cache::{cache::RenderedImageCache, error::RenderedCacheError},
        types::rendered_image::RenderedImage,
    },
    domain::value_object::image_id::image_id::ImageId,
};

pub struct RenderedImageCacheInMemory {
    caches: Mutex<HashMap<ImageId, RenderedImage>>,
}

impl RenderedImageCache for RenderedImageCacheInMemory {
    fn save(&self, image: RenderedImage) {
        let mut caches = self
            .caches
            .lock()
            .expect("RenderedImageCache Mutex is Poisoned");
        caches.insert(image.image_id(), image);
    }

    fn take(&self, id: ImageId) -> Result<RenderedImage, RenderedCacheError> {
        let mut caches = self
            .caches
            .lock()
            .expect("RenderedImageCache Mutex is Poisoned");
        caches.remove(&id).ok_or(RenderedCacheError::ImageNotFound)
    }
}

impl RenderedImageCacheInMemory {
    pub fn new() -> Self {
        Self {
            caches: Mutex::new(HashMap::new()),
        }
    }
}
