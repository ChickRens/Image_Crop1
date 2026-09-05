use image::{
    RgbaImage,
    imageops::{self, FilterType},
};
use webp::Encoder;

use crate::{
    application::{
        interface::preview_image_generator::PreviewImageGenerator,
        types::preview_image::PreviewImage,
    },
    domain::{
        entity::image::Image,
        value_object::{image_data::ImageData, image_size::image_size::ImageSize},
    },
};

#[derive(Debug)]
pub struct WebPPreviewImageGenerator {
    long_side: f64
}

impl PreviewImageGenerator for WebPPreviewImageGenerator {
    fn generate(&self, image: &Image) -> (PreviewImage, f64) {
        let source_size = image.image_size();
        let scale = self.long_side / source_size.height().max(source_size.width()) as f64;
        let width = ((source_size.width() as f64 * scale).round() as u16).max(2);
        let height = ((source_size.height() as f64 * scale).round() as u16).max(2);
        let preview_size = ImageSize::new(height, width).expect("preview size is invalid");

        let rgba_image = RgbaImage::from_raw(
            source_size.width() as u32, 
            source_size.height() as u32,
            image.image_data().image().clone(),
        )
        .expect("image data is not valid RGBA data");

        let resized = imageops::resize(
            &rgba_image,
            width as u32,
            height as u32,
            FilterType::Nearest,
        );

        let encoder = Encoder::from_rgba(
            resized.as_raw(),
            resized.width(),
            resized.height(),
        );
        let webp = encoder.encode(80.0).to_vec();

        (PreviewImage::new(Image::new(
            ImageData::new(webp),
            *image.image_id(),
            preview_size,
        )), scale)
    }
}

impl WebPPreviewImageGenerator {
    pub fn new(preview_image_long_side: u16) -> Self {
        Self { long_side: preview_image_long_side as f64 }
    }
}
