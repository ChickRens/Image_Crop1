#[cfg(test)]
mod image_repository_in_memory_test {
    use crate::domain::entity::image::Image;
    use crate::domain::repository::image_repository::ImageRepository;
    use crate::domain::value_object::{
        image_data::ImageData, image_id::ImageId, image_kind::ImageKind, image_size::ImageSize,
    };
    use crate::infrastructure::repository::image_repository::ImageRepositoryInMemory;

    #[test]
    fn test_normal_get() {
        let mut repo = ImageRepositoryInMemory::new();

        let image_id1 = ImageId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();

        let image1 = Image::new(
            ImageData::new(vec![0, 1, 2, 3]),
            image_id1.clone(),
            ImageSize::new(2, 2).unwrap(),
        );
        repo.save(image1.clone(), ImageKind::Original);

        let got1 = repo.get(&image_id1, ImageKind::Original).unwrap();

        assert_eq!(got1, image1);
    }

    #[test]
    fn test_unknown_image_get() {
        let mut repo = ImageRepositoryInMemory::new();

        let image_id1 = ImageId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();
        let image_id2 = ImageId::from_str("12345678-9abc-def0-1234-56789abcdef0").unwrap();

        let image1 = Image::new(
            ImageData::new(vec![255, 255, 255, 255, 0, 0, 0, 0, 128, 128, 128, 128]),
            image_id1.clone(),
            ImageSize::new(2, 2).unwrap(),
        );
        repo.save(image1, ImageKind::Original);

        let got = repo.get(&image_id2, ImageKind::Original);

        assert_eq!(got, None);
    }

    #[test]
    fn test_overwrite_save_existing_image() {
        let mut repo = ImageRepositoryInMemory::new();

        let image_id = ImageId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();

        let old_image = Image::new(
            ImageData::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 9, 10]),
            image_id.clone(),
            ImageSize::new(2, 2).unwrap(),
        );
        let new_image = Image::new(
            ImageData::new(vec![10, 9, 7, 6, 5, 4, 3, 2, 1, 0]),
            image_id.clone(),
            ImageSize::new(2, 2).unwrap(),
        );

        repo.save(old_image.clone(), ImageKind::Original);
        repo.save(new_image.clone(), ImageKind::Original);

        let got = repo.get(&image_id, ImageKind::Original);

        assert_eq!(got, Some(new_image));
    }

    #[test]
    fn test_multi_image() {
        let mut repo = ImageRepositoryInMemory::new();

        let image_id1 = ImageId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();
        let image_id2 = ImageId::from_str("12345678-9abc-def0-1234-56789abcdef0").unwrap();
        let image_id3 = ImageId::from_str("00000000-0000-0000-0000-000000000000").unwrap();

        let image1 = Image::new(
            ImageData::new(vec![0, 0, 0, 0, 0]),
            image_id1.clone(),
            ImageSize::new(2, 2).unwrap(),
        );
        let image2 = Image::new(
            ImageData::new(vec![2, 2, 2, 2, 2]),
            image_id2.clone(),
            ImageSize::new(2, 2).unwrap(),
        );
        let image3 = Image::new(
            ImageData::new(vec![4, 4, 4, 4, 4]),
            image_id3.clone(),
            ImageSize::new(2, 2).unwrap(),
        );

        repo.save(image1.clone(), ImageKind::Original);
        repo.save(image2.clone(), ImageKind::Original);
        repo.save(image3.clone(), ImageKind::Original);

        let got1 = repo.get(&image_id1, ImageKind::Original);
        let got2 = repo.get(&image_id2, ImageKind::Original);
        let got3 = repo.get(&image_id3, ImageKind::Original);

        assert_eq!(got1, Some(image1));
        assert_eq!(got2, Some(image2));
        assert_eq!(got3, Some(image3));
    }
}
