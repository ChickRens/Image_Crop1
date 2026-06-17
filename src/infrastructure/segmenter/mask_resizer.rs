use ndarray::Array4;

pub struct SAM2MaskResizer;

impl SAM2MaskResizer {
    pub fn resize_mask(mask: &Array4<f32>, target_h: usize, target_w: usize) -> Array4<f32> {
        let (batch, channel, src_h, src_w) = mask.dim();
        println!("batch       : {:?}", batch);
        println!("channel     : {:?}", channel);
        println!("mask_height : {:?}", src_h);
        println!("mask_width  : {:?}", src_w);

        let mut output = Array4::<f32>::zeros((batch, channel, target_h, target_w));

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

                let upper_left = mask[[0, 0, upper_left_y, upper_left_x]];
                let upper_right = mask[[0, 0, upper_left_y, lower_right_x]];
                let lower_left = mask[[0, 0, lower_right_y, upper_left_x]];
                let lower_right = mask[[0, 0, lower_right_y, lower_right_x]];

                let value = (1.0 - dx) * (1.0 - dy) * upper_left
                    + dx * (1.0 - dy) * upper_right
                    + (1.0 - dx) * dy * lower_left
                    + dx * dy * lower_right;

                output[[0, 0, y, x]] = value
                // output[[0, 0, y, x]] = mask[[0, 0, src_y, src_x]];
            }
        }
        output
    }
}
