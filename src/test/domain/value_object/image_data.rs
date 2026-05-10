#[cfg(test)]
mod image_data_tests {
    use crate::domain::value_object::image_data::ImageData;

    #[test]
    fn test_image_data_new_and_into_image() {
        let image_data = ImageData::new(vec![1, 2, 3]);
        assert_eq!(image_data.image(), &vec![1, 2, 3]);
        assert_eq!(image_data.clone().into_image(), vec![1, 2, 3]);
    }
}
