use ndarray::Array2;

pub struct SAM2MaskResizer;

impl SAM2MaskResizer {
    pub fn resize_mask(mask: &Array2<f32>, target_h: usize, target_w: usize) -> Array2<f32> {
        let (src_h, src_w) = mask.dim();

        let mut output = Array2::<f32>::zeros((target_h, target_w));

        for y in 0..target_h {
            let src_y = y * src_h / target_h;

            for x in 0..target_w {
                let src_x = x * src_w / target_w;

                output[[y, x]] = mask[[src_y, src_x]];
            }
        }
        output
    }
}
