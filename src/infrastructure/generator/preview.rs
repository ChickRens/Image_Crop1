use std::time::Instant;

use fast_image_resize as fr;
use image::{
    RgbaImage,
};
use webp::{Encoder, WebPConfig};

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

        let start = Instant::now();

        let mut dst_image = fr::images::Image::new(width as u32, height as u32, fr::PixelType::U8x4);
        let options = fr::ResizeOptions::new().resize_alg(fr::ResizeAlg::Nearest);

        let mut resizer = fr::Resizer::new();
        resizer.resize(&rgba_image, &mut dst_image, &options).expect("Resize Failed in PreviewGenerator");

        let end = start.elapsed();
        println!("Resize in PreviewGenerator: {:?}", end);

        let start = Instant::now();
        let dst_vec = dst_image.into_vec();
        let encoder = Encoder::from_rgba(
            &dst_vec,
            width as u32,
            height as u32,
        );

        let mut config = WebPConfig::new().unwrap();
        config.quality = 75.0;
        config.method = 0;
        config.thread_level = 1;
        config.alpha_quality = 50;
        config.alpha_filtering = 0;
        config.alpha_compression = 1; // まずは圧縮ありのままで様子見
        config.filter_strength = 0;
        config.segments = 1;
        config.sns_strength = 0;

        let webp = encoder.encode_advanced(&config).expect("Encode Failed").to_vec();
        let end = start.elapsed();
        println!("Encode in PreviewGenerator: {:?}", end);

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
