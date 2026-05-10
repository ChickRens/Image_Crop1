#[cfg(test)]
mod mask_tests {
    use crate::domain::value_object::{image_size::ImageSize, mask::Mask};

    #[test]
    fn test_mask_new_and_size() {
        let size = ImageSize::new(5, 5).unwrap();
        let mask = Mask::new(size.clone());
        assert_eq!(mask.mask_size(), &size);
    }
}
