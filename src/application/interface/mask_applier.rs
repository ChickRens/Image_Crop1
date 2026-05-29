use crate::application::types::mask::Mask;
use crate::domain::entity::image::Image;

pub trait MaskApplier {
    fn apply(original: &Image, mask: &Mask) -> Vec<u8>;
}