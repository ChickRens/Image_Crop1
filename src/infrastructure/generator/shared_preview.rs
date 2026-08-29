use std::sync::Arc;

use crate::{application::{interface::preview_image_generator::PreviewImageGenerator, types::preview_image::PreviewImage}, domain::entity::image::Image, infrastructure::generator::preview::WebPPreviewImageGenerator};

#[derive(Debug, Clone)]
pub struct SharedPreviewGenerator {
    generator: Arc<WebPPreviewImageGenerator>
}

impl PreviewImageGenerator for SharedPreviewGenerator {
    fn generate(&self, image: &Image) -> PreviewImage {
        self.generator.generate(image)
    }
}

impl SharedPreviewGenerator {
    pub fn new(preview_image_long_side: u16) -> Self {
        Self { generator: Arc::new(WebPPreviewImageGenerator::new(preview_image_long_side)) }
    }
}