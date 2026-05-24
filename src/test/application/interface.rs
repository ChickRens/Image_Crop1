#[cfg(test)]
mod interface_tests {
    use uuid::Uuid;

    use crate::application::errors::loading_errors::LoadingErrors;
    use crate::application::errors::segmentation_error::SegmentationErrors;
    use crate::application::interface::image_loader::ImageLoader;
    use crate::application::interface::image_segmenter::ImageSegmenter;
    use crate::application::types::{loaded_image::LoadedImage, segmented_image::SegmentedImage};
    use crate::domain::entity::image::Image;
    use crate::domain::value_object::{
        image_data::ImageData, image_id::ImageId, image_size::ImageSize, point::Point,
    };

    struct DummyLoader;
    impl ImageLoader for DummyLoader {
        fn load(&self, data: Vec<u8>) -> Result<LoadedImage, LoadingErrors> {
            let image = Image::new(
                ImageData::new(data),
                ImageId::from_uuid(Uuid::new_v4()),
                ImageSize::new(2, 2).unwrap(),
                0,
            );
            Ok(LoadedImage::new(image))
        }
    }

    struct DummySegmenter;
    impl ImageSegmenter for DummySegmenter {
        fn segment(
            &mut self,
            image: Image,
            _points: &[Point],
        ) -> Result<SegmentedImage, SegmentationErrors> {
            Ok(SegmentedImage::new(image))
        }
    }

    #[test]
    fn test_dummy_image_loader_implements_trait() {
        let loader = DummyLoader;
        let result = loader.load(vec![1, 2, 3]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_dummy_image_segmenter_implements_trait() {
        let image = Image::new(
            ImageData::new(vec![1, 2, 3]),
            ImageId::from_uuid(Uuid::new_v4()),
            ImageSize::new(2, 2).unwrap(),
            0,
        );
        let mut segmenter = DummySegmenter;
        let result = segmenter.segment(
            image,
            &[Point::new(Coordinate::new(4, 4), PointLabel::FOREGROUND)],
        );
        let _ = result.unwrap().into_image();
    }
}
