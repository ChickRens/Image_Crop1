#[cfg(test)]
mod image_tests {
    use uuid::Uuid;

    use crate::domain::entity::image::Image;
    use crate::domain::value_object::{
        image_data::ImageData,
        image_id::ImageId,
        image_size::{ImageSize, SizeErrorType},
    };

    #[test]
    fn test_create_normal_image() {
        let id: ImageId = ImageId::from_uuid(Uuid::new_v4());
        let size_res: Result<ImageSize, SizeErrorType> = ImageSize::new(3000, 3000);

        let size = size_res.expect("failed size generate");

        let image = Image::new(
            ImageData::new(vec![1, 2, 3]),
            id.clone(),
            size.clone(),
        );

        assert_eq!(image.image_id(), &id);
        assert_eq!(image.image_size(), &size);
    }
}
