use crate::{application::types::preview_image::PreviewImage, domain::entity::image::Image};

pub trait PreviewImageGenerator {
    fn generate(image: Image) -> PreviewImage;
}