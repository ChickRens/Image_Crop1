use ndarray::Array4;

pub struct SAM2MaskResizer;

impl SAM2MaskResizer {
    pub fn resize_mask(mask: &Array4<f32>, target_h: usize, target_w: usize) -> Array4<f32> {
        let (batch, channel, src_h, src_w) = mask.dim();

        let mut output = Array4::<f32>::zeros((batch, channel, target_h, target_w));

        for y in 0..target_h {
            let src_y = y * src_h / target_h;

            for x in 0..target_w {
                let src_x = x * src_w / target_w;

                output[[0, 0, y, x]] = mask[[0, 0, src_y, src_x]];
            }
        }
        output
    }
}
