#[cfg(test)]
mod segmenter_tests {
    use image::RgbaImage;
    use ndarray::Array4;

    use crate::{
        domain::{
            entity::image::Image,
            value_object::{image_data::ImageData, image_id::ImageId, image_size::ImageSize},
        },
        infrastructure::segmenter::mask_applier::SAM2MaskApplier,
    };

    fn _create_5x5_rgb_image() -> Image {
        let image_pixels: [u8; 75] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0,
            0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ];
        Image::new(
            ImageData::new(image_pixels.to_vec()),
            ImageId::new(),
            ImageSize::new(5, 5).unwrap(),
        )
    }

    fn _create_x_shape_5x5_mask() -> Array4<f32> {
        use ndarray::Array4;
        let mask_pixels: [f32; 25] = [
            1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ];

        Array4::from_shape_vec((1, 1, 5, 5), mask_pixels.to_vec()).unwrap()
    }

    #[ignore]
    #[test]
    fn test_visualize_apply_result() {
        let original = _create_5x5_rgb_image();
        let mask = _create_x_shape_5x5_mask();

        let applied = SAM2MaskApplier::apply(&original, mask);

        let (_data, _id, size) = original.into_data();

        let img = RgbaImage::from_raw(size.width() as u32, size.height() as u32, applied).unwrap();

        img.save("x_shape.png").unwrap();
    }

    #[test]
    fn test_normal_apply() {
        let original = _create_5x5_rgb_image();
        let mask = _create_x_shape_5x5_mask();

        let applied = SAM2MaskApplier::apply(&original, mask);

        assert_eq!(100, applied.len());
        assert_eq!(
            applied,
            [
                0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 255,
                255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 255,
                255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255,
                255, 255, 255, 255, 255, 255, 0, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 255, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255
            ]
        );
    }
}
