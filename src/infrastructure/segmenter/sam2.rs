use crate::application::errors::segmentation_error::SegmentationErrors;
use crate::application::interface::image_segmenter::ImageSegmenter;
use crate::application::types::segmented_image::SegmentedImage;
use crate::domain::entity::image::Image;
use crate::domain::value_object::point::Point;
use image::DynamicImage;
use image::GenericImageView;
use ndarray::{Array, Array4, IxDyn};
use ort::session::Session;
use ort::session::builder::GraphOptimizationLevel;
use ort::value::{DynValueTypeMarker, Tensor, Value};

pub struct Sam2Segmenter {
    session: Session,
}

impl Sam2Segmenter {
    pub fn new(model_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        ort::init();

        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .commit_from_file(model_path)?;

        Ok(Self { session })
    }

    pub fn encode_image(
        &mut self,
        img: &DynamicImage,
    ) -> Result<Array<f32, IxDyn>, Box<dyn std::error::Error>> {
        // Load and preprocess image
        let img = img.resize_exact(224, 224, image::imageops::FilterType::Lanczos3);
        let img = img.to_rgb8();

        // Convert to tensor (1, 3, 224, 224) with normalization
        let mut tensor_data = Vec::with_capacity(224 * 224 * 3);
        for pixel in img.pixels() {
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;
            // Normalize with ImageNet mean and std
            let r_norm = (r - 0.485) / 0.229;
            let g_norm = (g - 0.456) / 0.224;
            let b_norm = (b - 0.406) / 0.225;
            tensor_data.push(r_norm);
            tensor_data.push(g_norm);
            tensor_data.push(b_norm);
        }

        let input_tensor = Array4::from_shape_vec((1, 3, 224, 224), tensor_data)?;

        // Run inference
        let input_name = self.session.inputs()[0].name();
        let dim = input_tensor.dim();
        let shape_slice = &[dim.0, dim.1, dim.2, dim.3];
        let shape_ref = unsafe { std::slice::from_raw_parts(shape_slice.as_ptr(), 4) };
        let data: Vec<f32> = input_tensor.iter().cloned().collect();
        let tensor: Tensor<f32> = Tensor::from_array((shape_ref, data))?;
        let inputs = vec![(
            input_name.to_string(),
            Value::<DynValueTypeMarker>::from(tensor),
        )];
        let outputs = self.session.run(inputs)?;
        let (shape, data) = outputs[0].try_extract_tensor::<f32>()?;
        let shape_vec = shape.iter().map(|&x| x as usize).collect::<Vec<usize>>();
        let embedding_array = Array::from_shape_vec(IxDyn(&shape_vec), data.to_vec())?;

        Ok(embedding_array)
    }
}

impl ImageSegmenter for Sam2Segmenter {
    fn segment(
        &self,
        image: Image,
        _points: Option<&Point>,
    ) -> Result<SegmentedImage, SegmentationErrors> {
        // For now, just return the image as segmented since full SAM2 requires decoder
        // TODO: Implement full segmentation with decoder
        Ok(SegmentedImage::new(image))
    }
}
