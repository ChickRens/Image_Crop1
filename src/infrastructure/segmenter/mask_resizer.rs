use std::time;

use ndarray::{Array4, Axis};

pub struct SAM2MaskResizer;

impl SAM2MaskResizer {
    pub fn resize_mask(mask: &Array4<f32>, target_h: usize, target_w: usize) -> Array4<f32> {
        let (batch, channel, src_h, src_w) = mask.dim();
        let mask = mask.index_axis(Axis(0), 0);
        let mask = mask.index_axis(Axis(0), 0);

        let input = mask.as_slice().unwrap();

        println!("batch       : {:?}", batch);
        println!("channel     : {:?}", channel);
        println!("mask_height : {:?}", src_h);
        println!("mask_width  : {:?}", src_w);

        let mut output = Array4::<f32>::zeros((1, 1, target_h, target_w));
        let mut output_view = output.index_axis_mut(Axis(0), 0);
        let mut output_view = output_view.index_axis_mut(Axis(0), 0);

        let output_slice = output_view.as_slice_mut().unwrap();

        for y in 0..target_h {
            let src_y = (y as f32 + 0.5) * src_h as f32 / target_h as f32 - 0.5;
            let src_y = src_y.clamp(0.0, (src_h - 1) as f32);

            let upper_left_y = src_y.floor() as usize;
            let lower_right_y = (upper_left_y + 1).min(src_h - 1);
            let dy = src_y - upper_left_y as f32;

            for x in 0..target_w {
                let src_x = (x as f32 + 0.5) * src_w as f32 / target_w as f32 - 0.5;
                let src_x = src_x.clamp(0.0, (src_w - 1) as f32);

                let upper_left_x = src_x.floor() as usize;
                let lower_right_x = (upper_left_x + 1).min(src_w - 1);

                let dx = src_x - upper_left_x as f32;

                let upper_row = upper_left_y * src_w;
                let lower_row = lower_right_y * src_w;

                let upper_left = input[upper_row + upper_left_x];
                let upper_right = input[upper_row + lower_right_x];
                let lower_left = input[lower_row + upper_left_x];
                let lower_right = input[lower_row + lower_right_x];

                let value = (1.0 - dx) * (1.0 - dy) * upper_left
                    + dx * (1.0 - dy) * upper_right
                    + (1.0 - dx) * dy * lower_left
                    + dx * dy * lower_right;

                output_slice[y * target_w + x] = value
                // output[[0, 0, y, x]] = mask[[0, 0, src_y, src_x]];
            }
        }

        output
    }
}
