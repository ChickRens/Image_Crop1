#[cfg(test)]
mod segmenter_tests {
    use image::RgbaImage;

    use crate::{
        application::{
            interface::image_segmenter::segmenter::ImageSegmenter,
            types::{
                editing_session::session::{CommonEditingSession, EditingSession},
                inference_context_history::InferenceContextHistory,
                point_history::PointHistory,
            },
        },
        domain::{
            entity::image::image::Image,
            value_object::{
                coordinate::Coordinate,
                image_data::ImageData,
                image_id::image_id::ImageId,
                image_size::image_size::ImageSize,
                point::{Point, PointLabel},
            },
        },
        infrastructure::segmenter::sam2::Sam2Segmenter,
    };

    fn _create_5x5_rgb() -> Image {
        let image_pixels: [u8; 75] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0,
            0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ];
        Image::new(
            ImageData::new(image_pixels.to_vec()),
            ImageId::new(),
            ImageSize::new(5, 5).unwrap(),
        )
    }

    #[test]
    fn test_sam2_module_exists() {
        let model_dir = "models";
        let segmenter = Sam2Segmenter::new(model_dir);

        assert!(segmenter.is_ok())
    }

    #[test]
    fn test_sam2_module_not_exists() {
        let segmenter = Sam2Segmenter::new("invalid_dir");

        assert!(segmenter.is_err())
    }

    #[test]
    fn test_segmenter_preparing() {
        let model_dir = "models";
        let segmenter = Sam2Segmenter::new(model_dir).unwrap();

        let image = _create_5x5_rgb();

        let static_context = segmenter.prepare_static_context(&image);
        let inference_context = segmenter.prepare_inference_context(&image);

        assert!(static_context.is_ok());
        assert!(inference_context.is_ok());
    }

    #[ignore]
    #[test]
    fn test_visualize_segmenter_segment_with_single_point() {
        let model_dir = "models";
        let segmenter = Sam2Segmenter::new(model_dir).unwrap();

        let image = _create_5x5_rgb();

        let static_context = segmenter.prepare_static_context(&image).unwrap();
        let inference_context = segmenter.prepare_inference_context(&image).unwrap();

        let editing_session = CommonEditingSession::new(
            PointHistory::new(30),
            static_context,
            InferenceContextHistory::new(30),
            inference_context,
        );

        let segmented_res = segmenter.segment(
            &image,
            editing_session.static_context(),
            editing_session.inference_context(),
            &editing_session.points_with(Point::new(Coordinate::new(2, 2), PointLabel::FOREGROUND)),
        );

        match &segmented_res {
            Ok(_) => assert!(true),
            Err(e) => println!("{:?}", e),
        }

        assert!(segmented_res.is_ok());

        let (_, segmented_image) = segmented_res.unwrap();

        let (data, size) = segmented_image.into_image_and_size();
        assert_eq!(size, ImageSize::new(5, 5).unwrap());

        let img = RgbaImage::from_raw(size.width() as u32, size.height() as u32, data.into_image())
            .unwrap();

        img.save("test_segmenter_segment_with_single_point_result.png")
            .unwrap();
    }

    #[test]
    fn test_segmenter_segment() {
        let model_dir = "models";
        let segmenter = Sam2Segmenter::new(model_dir).unwrap();

        let image = _create_5x5_rgb();

        let static_context = segmenter.prepare_static_context(&image).unwrap();
        let inference_context = segmenter.prepare_inference_context(&image).unwrap();

        let editing_session = CommonEditingSession::new(
            PointHistory::new(40),
            static_context,
            InferenceContextHistory::new(40),
            inference_context,
        );

        let segmented_res = segmenter.segment(
            &image,
            editing_session.static_context(),
            editing_session.inference_context(),
            &editing_session.points_with(Point::new(Coordinate::new(2, 2), PointLabel::FOREGROUND)),
        );

        match &segmented_res {
            Ok(_) => assert!(true),
            Err(e) => println!("{:?}", e),
        }

        assert!(segmented_res.is_ok());

        let (_, segmented_image) = segmented_res.unwrap();

        let (data, size) = segmented_image.into_image_and_size();
        assert_eq!(size, ImageSize::new(5, 5).unwrap());
        assert_eq!(data.image().len(), 100)
    }
}
