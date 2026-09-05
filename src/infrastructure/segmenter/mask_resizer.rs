use ndarray::{Array4, ArrayView4, Axis};
use rayon::{iter::{IndexedParallelIterator, ParallelIterator}, slice::ParallelSliceMut};

pub struct SAM2MaskResizer;

impl SAM2MaskResizer {
    pub fn resize_mask(mask: &ArrayView4<f32>, target_h: usize, target_w: usize) -> Array4<f32> {
        let (batch, channel, src_h, src_w) = mask.dim();

        debug_assert_eq!(batch, 1);
        debug_assert_eq!(channel, 1);

        let mask = mask.index_axis(Axis(0), 0);
        let mask = mask.index_axis(Axis(0), 0);

        let input = mask
            .as_slice()
            .expect("Resizer requires contiguous ArrayView4");

        let mut output = Array4::<f32>::zeros((1, 1, target_h, target_w));
        let mut output_view = output.index_axis_mut(Axis(0), 0);
        let mut output_view = output_view.index_axis_mut(Axis(0), 0);

        let output_slice = output_view
            .as_slice_mut()
            .expect("Resizer requires contiguous ArrayView4");

        output_slice
            .par_chunks_exact_mut(target_w)
            .enumerate()
            .for_each(|(y, row)|
            {
                let src_y = (y as f32 + 0.5) * src_h as f32 / target_h as f32 - 0.5;
                let src_y = src_y.clamp(0.0, (src_h - 1) as f32);

                let upper_y = src_y.floor() as usize;
                let lower_y = (upper_y + 1).min(src_h - 1);
                let dy = src_y - upper_y as f32;

                let upper_row = upper_y * src_w;
                let lower_row = lower_y * src_w;

                for x in 0..target_w {
                    let src_x = (x as f32 + 0.5) * src_w as f32 / target_w as f32 - 0.5;
                    let src_x = src_x.clamp(0.0, (src_h - 1) as f32);

                    let left_x = src_x.floor() as usize;
                    let right_x = (left_x + 1).min(src_w - 1);

                    let dx = src_x - left_x as f32;

                    let upper_left = input[upper_row + left_x];
                    let upper_right = input[upper_row + right_x];
                    let lower_left = input[lower_row + left_x];
                    let lower_right = input[lower_row + right_x];

                    row[x] = (1.0 - dx) * (1.0 - dy) * upper_left
                        + dx * (1.0 - dy) * upper_right
                        + (1.0 - dx) * dy * lower_left
                        + dx * dy * lower_right;
                }
            });

            output
    }
}
