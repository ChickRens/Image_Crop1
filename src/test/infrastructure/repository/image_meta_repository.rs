#[cfg(test)]
mod image_meta_repository_in_memory_test {
    use crate::{domain::{entity::image::Image, repository::image_repository::ImageRepository, value_object::{image_id::ImageId, image_size::ImageSize}}, infrastructure::repository::image_meta_repository::ImageMetaRepositoryInMemory};
    #[test]
    fn test_normal_get() {
        let mut repo=ImageMetaRepositoryInMemory::new();

        let image_id = ImageId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();

        let meta1=Image::new(image_id, ImageSize::new(50, 100).unwrap(), 16);
        repo.save(meta1.clone());

        let meta = repo.get(&image_id).unwrap();

        assert_eq!(meta, meta1);
    }

    #[test]
    fn test_unknown_meta_get(){
        let mut repo=ImageMetaRepositoryInMemory::new();

        let image_id1 = ImageId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();
        let image_id2 = ImageId::from_str("12345678-9abc-def0-1234-56789abcdef0").unwrap();

        let meta1=Image::new(image_id1, ImageSize::new(50, 100).unwrap(), 16);
        repo.save(meta1.clone());

        let meta = repo.get(&image_id2);

        assert_eq!(meta, None);
    }

    #[test]
    fn test_overwrite_save_existing_meta(){
        let mut repo=ImageMetaRepositoryInMemory::new();

        let image_id = ImageId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();

        let old_meta=Image::new(image_id, ImageSize::new(50, 100).unwrap(), 16);
        let new_meta=Image::new(image_id, ImageSize::new(200, 500).unwrap(), 32);
        repo.save(old_meta.clone());
        repo.save(new_meta.clone());

        let got_meta = repo.get(&image_id);

        assert_eq!(got_meta, Some(new_meta));
    }

    #[test]
    fn test_multi_meta(){
        let mut repo=ImageMetaRepositoryInMemory::new();

        let image_id1 = ImageId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();
        let image_id2 = ImageId::from_str("12345678-9abc-def0-1234-56789abcdef0").unwrap();
        let image_id3 = ImageId::from_str("00000000-0000-0000-0000-000000000000").unwrap();

        let meta1=Image::new(image_id1, ImageSize::new(50, 100).unwrap(), 16);
        let meta2=Image::new(image_id2, ImageSize::new(100, 200).unwrap(), 16);
        let meta3=Image::new(image_id3, ImageSize::new(150, 300).unwrap(), 16);

        repo.save(meta1.clone());
        repo.save(meta2.clone());
        repo.save(meta3.clone());

        let got1 = repo.get(&image_id1);
        let got2 = repo.get(&image_id2);
        let got3 = repo.get(&image_id3);

        assert_eq!(got1,Some(meta1));
        assert_eq!(got2,Some(meta2));
        assert_eq!(got3,Some(meta3));
    }
}