#[cfg(test)]
mod types_tests {
    use uuid::Uuid;

    use crate::application::types::{loaded_image::LoadedImage, segmented_image::SegmentedImage};
    use crate::domain::entity::image::Image;
    use crate::domain::value_object::{
        image_data::ImageData, image_id::ImageId, image_size::ImageSize,
    };

    #[test]
    fn test_loaded_image_contains_image() {
        let image = Image::new(
            ImageData::new(vec![1, 2, 3]),
            ImageId::from_uuid(Uuid::new_v4()),
            ImageSize::new(2, 2).unwrap(),
        );

        let loaded = LoadedImage::new(image.clone());
        assert_eq!(loaded.into_image(), image);
    }

    #[test]
    fn test_segmented_image_contains_image() {
        let image = Image::new(
            ImageData::new(vec![4, 5, 6]),
            ImageId::from_uuid(Uuid::new_v4()),
            ImageSize::new(2, 2).unwrap(),
        );

        let segmented = SegmentedImage::new(image.clone());
        assert_eq!(segmented.into_image(), image);
    }
}
