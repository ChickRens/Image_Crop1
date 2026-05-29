use ndarray::prelude::{ArrayBase, Dim};
use ndarray::{ViewRepr, s};

use crate::application::interface::mask_applier::MaskApplier;
use crate::application::types::mask::Mask;
use crate::domain::entity::image::Image;

pub struct SAM2MaskApplier;

impl MaskApplier for SAM2MaskApplier {
    fn apply(original: &Image, mask: &Mask) -> Vec<u8> {
        let mask_4d: ArrayBase<ViewRepr<&f32>, Dim<[usize; 4]>, f32> = mask.view();
        let mask_2d: ArrayBase<ViewRepr<&f32>, Dim<[usize; 2]>, f32> =
            mask_4d.slice(s![0, 0, .., ..]);

        let original_data = original.image_data().image();
        let mut output: Vec<u8> = Vec::with_capacity(original_data.len());

        let height = original.image_size().height() as usize;
        let width = original.image_size().width() as usize;

        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) * 4;

                let mask_value = &mask_2d[[y, x]];

                if *mask_value < 0.7 as f32 {
                    output[index + 3] = 0;
                    continue;
                }

                output[index] = original_data[index];
                output[index + 1] = original_data[index + 1];
                output[index + 2] = original_data[index + 2];
                output[index + 3] = 1
            }
        }

        output
    }
}
