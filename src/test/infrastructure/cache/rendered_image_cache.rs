#[cfg(test)]
mod rendered_image_cache_test {
    use crate::{
        application::{
            interface::rendered_image_cache::{
                cache::RenderedImageCache, error::RenderedCacheError,
            },
            types::rendered_image::RenderedImage,
        },
        domain::value_object::{
            image_data::ImageData, image_id::image_id::ImageId, image_size::image_size::ImageSize,
        },
        infrastructure::cache::rendered_image_cache::RenderedImageCacheInMemory,
    };

    #[test]
    fn test_normal_get() {
        let cache = RenderedImageCacheInMemory::new();
        let image_id = ImageId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();
        let image = RenderedImage::new(
            ImageData::new(vec![0, 1, 2, 3]),
            image_id.clone(),
            ImageSize::new(2, 2).unwrap(),
        );

        cache.save(image.clone());

        let got = cache.take(image_id.clone()).unwrap();

        assert_eq!(got, image);
    }

    #[test]
    fn test_unknown_image_get() {
        let cache = RenderedImageCacheInMemory::new();
        let unknown_image_id = ImageId::from_str("12345678-9abc-def0-1234-56789abcdef0").unwrap();

        let got = cache.take(unknown_image_id);

        assert_eq!(got, Err(RenderedCacheError::ImageNotFound));
    }

    #[test]
    fn test_overwrite_save_existing_image() {
        let cache = RenderedImageCacheInMemory::new();
        let image_id = ImageId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();

        let old_image = RenderedImage::new(
            ImageData::new(vec![0, 1, 2, 3]),
            image_id.clone(),
            ImageSize::new(2, 2).unwrap(),
        );
        let new_image = RenderedImage::new(
            ImageData::new(vec![9, 8, 7, 6]),
            image_id.clone(),
            ImageSize::new(2, 2).unwrap(),
        );

        cache.save(old_image);
        cache.save(new_image.clone());

        let got = cache.take(image_id).unwrap();

        assert_eq!(got, new_image);
    }
}
