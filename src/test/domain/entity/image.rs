#[cfg(test)]
mod image_tests {
    use uuid::Uuid;

    use crate::domain::entity::image_meta::ImageMeta;
    use crate::domain::value_object::image_id::ImageId;
    use crate::domain::value_object::image_size::{ImageSize, SizeErrorType};

    #[test]
    fn test_create_normal_image() {
        let id: ImageId = ImageId::from_uuid(Uuid::new_v4());
        let size_res: Result<ImageSize, SizeErrorType> = ImageSize::new(3000, 3000);

        let size = size_res.expect("failed size generate");
        const MAX_HISTORY: usize = 50;

        let image = ImageMeta::new(id.clone(), size.clone(), MAX_HISTORY);

        assert_eq!(image.image_id(), &id);
        assert_eq!(image.image_size(), &size);
    }
}
