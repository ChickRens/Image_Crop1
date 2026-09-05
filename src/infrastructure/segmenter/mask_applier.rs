use ndarray::{ArrayView4, s};
use rayon::prelude::*;

use crate::domain::entity::image::Image;

pub struct SAM2MaskApplier;

impl SAM2MaskApplier {
    pub fn apply(original: &Image, mask: ArrayView4<f32>) -> Vec<u8> {
        let mask_2d = mask.slice(s![0, 0, .., ..]);
        let original_data = original.image_data().image();

        let mut output = original_data.to_vec();

        debug_assert!(mask_2d.as_slice().is_some());

        let mask_slice = mask_2d
            .as_slice()
            .expect("Applier requires contiguous ArrayView4");

        output
            .par_chunks_exact_mut(4)
            .zip(mask_slice.par_iter())
            .for_each(|(pixel, pixel_alpha)| {
                pixel[3] = (pixel_alpha * 255.0).clamp(0.0, 255.0) as u8;
            });

        output
    }
}
