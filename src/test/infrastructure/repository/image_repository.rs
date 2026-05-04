#[cfg(test)]
mod image_meta_repository_in_memory_test {
    use crate::application::repository::image_repository::ImageRepository;
    use crate::infrastructure::repository::image_repository::ImageRepositoryInMemory;
    use crate::domain::value_object::image_id::ImageId;
    use crate::application::types::image::Image;

    #[test]
    fn test_normal_get() {
        let mut repo=ImageRepositoryInMemory::new();

        let image_id1 = ImageId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();

        let image1=Image::new(vec![0,1,2,3]);
        repo.save(image1.clone(), image_id1);

        let got1 = repo.get(&image_id1).unwrap();

        assert_eq!(got1, image1);
    }

    #[test]
    fn test_unknown_image_get(){
        let mut repo=ImageRepositoryInMemory::new();

        let image_id1 = ImageId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();
        let image_id2 = ImageId::from_str("12345678-9abc-def0-1234-56789abcdef0").unwrap();

        let image1=Image::new(vec![255,255,255,255,0,0,0,0,128,128,128,128]);
        repo.save(image1, image_id1);

        let got = repo.get(&image_id2);

        assert_eq!(got, None);
    }

    #[test]
    fn test_overwrite_save_existing_image(){
        let mut repo=ImageRepositoryInMemory::new();

        let image_id = ImageId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();

        let old_image=Image::new(vec![0,1,2,3,4,5,6,7,9,10]);
        let new_image=Image::new(vec![10,9,7,6,5,4,3,2,1,0]);

        repo.save(old_image.clone(), image_id);
        repo.save(new_image.clone(), image_id);

        let got = repo.get(&image_id);

        assert_eq!(got, Some(new_image));
    }

    #[test]
    fn test_multi_image(){
        let mut repo=ImageRepositoryInMemory::new();

        let image_id1 = ImageId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();
        let image_id2 = ImageId::from_str("12345678-9abc-def0-1234-56789abcdef0").unwrap();
        let image_id3 = ImageId::from_str("00000000-0000-0000-0000-000000000000").unwrap();

        let image1=Image::new(vec![0,0,0,0,0]);
        let image2=Image::new(vec![2,2,2,2,2]);
        let image3=Image::new(vec![4,4,4,4,4]);

        repo.save(image1.clone(), image_id1);
        repo.save(image2.clone(), image_id2);
        repo.save(image3.clone(), image_id3);

        let got1 = repo.get(&image_id1);
        let got2 = repo.get(&image_id2);
        let got3 = repo.get(&image_id3);

        assert_eq!(got1,Some(image1));
        assert_eq!(got2,Some(image2));
        assert_eq!(got3,Some(image3));
    }
}