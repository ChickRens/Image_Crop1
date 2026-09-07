use std::sync::Arc;

use webp::Encoder;

use crate::{application::{interface::completed_image_generator::CompletedImageGenerator, types::completed_image::CompletedImage}, domain::{entity::image::Image, value_object::image_data::ImageData}};

#[derive(Debug)]
pub struct WebPCompletedImageGenerator;

impl CompletedImageGenerator for WebPCompletedImageGenerator {
    fn generate(&self, image: &Image) -> CompletedImage {
        let size = image.image_size();

        let encoder = Encoder::from_rgba(
            image.image_data().image(),
            size.width() as u32,
            size.height() as u32,
        );

        let webp = encoder.encode_lossless().to_vec();
        CompletedImage::new(
            Image::new(ImageData::new(webp), *image.image_id(), image.image_size().clone())
        )
    }
}

impl WebPCompletedImageGenerator {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct SharedWebPCompletedImageGenerator {
    generator: Arc<WebPCompletedImageGenerator>
}

impl CompletedImageGenerator for SharedWebPCompletedImageGenerator {
    fn generate(&self, image: &Image) -> CompletedImage {
        self.generator.generate(image)
    }
}

impl SharedWebPCompletedImageGenerator {
    pub fn new() -> Self {
        Self { generator: Arc::new(WebPCompletedImageGenerator::new()) }
    }
}