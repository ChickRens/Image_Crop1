#[cfg(test)]
mod image_loader_test {
    use crate::infrastructure::segmenter::mask_resizer::SAM2MaskResizer;
    use ndarray::Array4;

    const EPSILON: f32 = 1e-5;
    #[test]
    fn test_resize() {
        let mut mask = Array4::<f32>::zeros((1, 1, 2, 2));

        mask[[0, 0, 0, 0]] = 0.0;
        mask[[0, 0, 0, 1]] = 1.1;
        mask[[0, 0, 1, 0]] = 5.01;
        mask[[0, 0, 1, 1]] = 10.002;

        let output = SAM2MaskResizer::resize_mask(&mask, 2, 2);

        for y in 0..2 {
            for x in 0..2 {
                assert!((mask[[0, 0, y, x]] - output[[0, 0, y, x]]).abs() < EPSILON)
            }
        }
    }

    #[test]
    fn test_resized_mask_do_not_contain_invalid_value() {
        let mut mask = Array4::<f32>::zeros((1, 1, 2, 2));

        mask[[0, 0, 0, 0]] = 0.0;
        mask[[0, 0, 0, 1]] = 1.0;
        mask[[0, 0, 1, 0]] = 0.5;
        mask[[0, 0, 1, 1]] = 1.0;

        let output = SAM2MaskResizer::resize_mask(&mask, 100, 100);

        for y in 0..100 {
            for x in 0..100 {
                assert!((0.0..=1.0).contains(&output[[0, 0, y, x]]))
            }
        }
    }

    #[test]
    fn test_constant_mask_remains_constant() {
        let mut mask = Array4::<f32>::zeros((1, 1, 2, 2));

        mask[[0, 0, 0, 0]] = 5.0;
        mask[[0, 0, 0, 1]] = 5.0;
        mask[[0, 0, 1, 0]] = 5.0;
        mask[[0, 0, 1, 1]] = 5.0;

        let output = SAM2MaskResizer::resize_mask(&mask, 300, 400);

        for y in 0..300 {
            for x in 0..400 {
                assert!(((output[[0, 0, y, x]] - 5.0).abs()) < EPSILON)
            }
        }
    }

    #[test]
    fn test_resized_mask_is_expected_shape() {
        let mut mask = Array4::<f32>::zeros((1, 1, 2, 2));

        mask[[0, 0, 0, 0]] = 5.0;
        mask[[0, 0, 0, 1]] = 5.0;
        mask[[0, 0, 1, 0]] = 5.0;
        mask[[0, 0, 1, 1]] = 5.0;

        let output = SAM2MaskResizer::resize_mask(&mask, 3000, 500);

        assert_eq!(output.dim(), (1, 1, 3000, 500))
    }
}
