#[cfg(test)]
mod image_kind_tests {
    use crate::domain::value_object::image_kind::ImageKind;

    #[test]
    fn test_image_kind_variants() {
        assert_eq!(ImageKind::Original, ImageKind::Original);
        assert_eq!(ImageKind::Segmented, ImageKind::Segmented);
    }
}
