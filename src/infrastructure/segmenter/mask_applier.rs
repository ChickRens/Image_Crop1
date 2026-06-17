use ndarray::prelude::{ArrayBase, Dim};
use ndarray::{Array4, ViewRepr, s};

use crate::domain::entity::image::Image;

pub struct SAM2MaskApplier;

impl SAM2MaskApplier {
    pub fn apply(original: &Image, mask: Array4<f32>) -> Vec<u8> {
        let mask_4d: ArrayBase<ViewRepr<&f32>, Dim<[usize; 4]>, f32> = mask.view();
        let mask_2d: ArrayBase<ViewRepr<&f32>, Dim<[usize; 2]>, f32> =
            mask_4d.slice(s![0, 0, .., ..]);

        let original_data = original.image_data().image();
        let height = original.image_size().height() as usize;
        let width = original.image_size().width() as usize;

        let mut output: Vec<u8> = vec![0u8; width * height * 4];

        for y in 0..height {
            for x in 0..width {
                let rgba_index = (y * width + x) * 4;
                let rgb_index = (y * width + x) * 3;

                let mask_value = &mask_2d[[y, x]];

                output[rgba_index] = original_data[rgb_index];
                output[rgba_index + 1] = original_data[rgb_index + 1];
                output[rgba_index + 2] = original_data[rgb_index + 2];
                output[rgba_index + 3] = (*mask_value * 255.0).clamp(0.0, 255.0) as u8;
            }
        }

        output
    }
}
