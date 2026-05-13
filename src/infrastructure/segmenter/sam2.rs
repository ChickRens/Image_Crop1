use crate::application::errors::segmentation_error::SegmentationErrors;
use crate::application::interface::image_segmenter::{ImageSegmenter, ImageSegmenterPreparing};
use crate::application::types::segmented_image::SegmentedImage;
use crate::domain::entity::image::Image;
use crate::domain::value_object::point::Point;
use image::DynamicImage;
use ndarray::{s, Array, Array4, IxDyn};
use ort::session::builder::GraphOptimizationLevel;
use ort::session::Session;
use ort::value::{DynValueTypeMarker, Tensor, Value};
use std::collections::HashMap;
use std::sync::Arc;

type OrtResult<T> = Result<T, Box<dyn std::error::Error>>;

type OrtInputs = HashMap<String, Value<DynValueTypeMarker>>;

pub struct Sam2Segmenter {
    encoder_session: Session,
    prompt_encoder_session: Session,
    mask_decoder_session: Session,
    image_embedding: Option<Arc<Array<f32, IxDyn>>>,
}

impl Sam2Segmenter {
    pub fn new(model_dir: &str) -> OrtResult<Self> {
        ort::init();

        Ok(Self {
            encoder_session: Self::build_session(format!("{}/sam2.1_hiera_small_encoder.onnx", model_dir))?,
            prompt_encoder_session: Self::build_session(format!("{}/sam2.1_hiera_small_prompt_encoder.onnx", model_dir))?,
            mask_decoder_session: Self::build_session(format!("{}/sam2.1_hiera_small_mask_decoder.onnx", model_dir))?,
            image_embedding: None,
        })
    }

    fn build_session(path: String) -> OrtResult<Session> {
        Ok(Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .commit_from_file(path)?)
    }

    fn array_to_value(array: &Array<f32, IxDyn>) -> OrtResult<Value<DynValueTypeMarker>> {
        let shape = array.shape().iter().map(|&x| x as i64).collect::<Vec<_>>();
        let data = array.iter().cloned().collect::<Vec<_>>();
        Ok(Value::from(Tensor::from_array((shape.as_slice(), data))?))
    }

    fn zero_value(shape: &[i64]) -> OrtResult<Value<DynValueTypeMarker>> {
        let size = shape.iter().product::<i64>() as usize;
        let data = vec![0.0; size];
        Ok(Value::from(Tensor::from_array((shape, data))?))
    }

    fn run_session(session: &mut Session, inputs: OrtInputs) -> OrtResult<Array<f32, IxDyn>> {
        let outputs = session.run(inputs)?;
        let (_, value) = outputs
            .into_iter()
            .next()
            .ok_or("no outputs in the session result")?;
        let (shape, data) = value.try_extract_tensor::<f32>()?;
        let shape = shape.iter().map(|&x| x as usize).collect::<Vec<usize>>();
        Ok(Array::from_shape_vec(IxDyn(&shape), data.to_vec())?)
    }

    pub fn encode_image(&mut self, img: &DynamicImage) -> OrtResult<Array<f32, IxDyn>> {
        let img = img.resize_exact(1024, 1024, image::imageops::FilterType::Lanczos3);
        let img = img.to_rgb8();
        let mut tensor_data = Vec::with_capacity(1024 * 1024 * 3);

        for pixel in img.pixels() {
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;
            tensor_data.push((r - 0.485) / 0.229);
            tensor_data.push((g - 0.456) / 0.224);
            tensor_data.push((b - 0.406) / 0.225);
        }

        let input_tensor = Array4::from_shape_vec((1, 3, 1024, 1024), tensor_data)?;
        let input_name = {
            let inputs = self.encoder_session.inputs();
            inputs[0].name().to_string()
        };
        let shape = vec![1i64, 3, 1024, 1024];
        let tensor = Tensor::from_array((shape.as_slice(), input_tensor.iter().cloned().collect::<Vec<_>>()))?;
        let mut inputs = OrtInputs::new();
        inputs.insert(input_name, Value::from(tensor));

        Self::run_session(&mut self.encoder_session, inputs)
    }

    pub fn encode_prompts(
        &mut self,
        points: &[Point],
        orig_h: f32,
        orig_w: f32,
    ) -> OrtResult<(Array<f32, IxDyn>, Array<f32, IxDyn>)> {
        let num_points = points.len();
        let coords: Vec<f32> = points
            .iter()
            .flat_map(|point| {
                let x_norm = point.coordinate().x() as f32 / orig_w;
                let y_norm = point.coordinate().y() as f32 / orig_h;
                [x_norm * 1024.0, y_norm * 1024.0]
            })
            .collect();

        let labels: Vec<f32> = points
            .iter()
            .map(|point| match point.label() {
                crate::domain::value_object::point::PointLabel::BACKGROUND => 0.0,
                crate::domain::value_object::point::PointLabel::FOREGROUND => 1.0,
            })
            .collect();

        let mut inputs = OrtInputs::new();
        inputs.insert(
            "points_coords".to_string(),
            Self::array_to_value(&Array::from_shape_vec(IxDyn(&[1, num_points, 2]), coords)?)?,
        );
        inputs.insert(
            "points_labels".to_string(),
            Self::array_to_value(&Array::from_shape_vec(IxDyn(&[1, num_points]), labels)?)?,
        );
        inputs.insert("boxes".to_string(), Self::zero_value(&[1, 4])?);
        inputs.insert("masks".to_string(), Self::zero_value(&[1, 1, 1024, 1024])?);

        let outputs = self.prompt_encoder_session.run(inputs)?;
        let sparse_emb = outputs[0].try_extract_tensor::<f32>()?;
        let dense_emb = outputs[1].try_extract_tensor::<f32>()?;

        let sparse_array = Array::from_shape_vec(
            IxDyn(&sparse_emb.0.iter().map(|&x| x as usize).collect::<Vec<_>>() ),
            sparse_emb.1.to_vec(),
        )?;

        let dense_array = Array::from_shape_vec(
            IxDyn(&dense_emb.0.iter().map(|&x| x as usize).collect::<Vec<_>>() ),
            dense_emb.1.to_vec(),
        )?;

        Ok((sparse_array, dense_array))
    }

    pub fn decode_mask(
        &mut self,
        image_emb: &Array<f32, IxDyn>,
        sparse_emb: &Array<f32, IxDyn>,
        dense_emb: &Array<f32, IxDyn>,
    ) -> OrtResult<Array<f32, IxDyn>> {
        let mut inputs = OrtInputs::new();
        inputs.insert("image_embeddings".to_string(), Self::array_to_value(image_emb)?);
        inputs.insert("sparse_embeddings".to_string(), Self::array_to_value(sparse_emb)?);
        inputs.insert("dense_embeddings".to_string(), Self::array_to_value(dense_emb)?);

        Self::run_session(&mut self.mask_decoder_session, inputs)
    }
}

impl ImageSegmenter for Sam2Segmenter {
    fn segment(
        &mut self,
        image: Image,
        points: &[Point],
    ) -> Result<SegmentedImage, SegmentationErrors> {
        let image_emb = self
            .image_embedding
            .clone()
            .ok_or(SegmentationErrors::NotPrepared)?;

        let orig_h = image.image_size().height() as f32;
        let orig_w = image.image_size().width() as f32;
        let (sparse_emb, dense_emb) = self
            .encode_prompts(points, orig_h, orig_w)
            .map_err(|e| SegmentationErrors::InferenceError(format!("Prompt encoding failed: {}", e)))?;
        let mask = self
            .decode_mask(image_emb.as_ref(), &sparse_emb, &dense_emb)
            .map_err(|e| SegmentationErrors::InferenceError(format!("Mask decoding failed: {}", e)))?;

        let mask_2d = mask.slice(s![0, 0, .., ..]).to_owned();
        Ok(SegmentedImage::with_mask(image, mask_2d))
    }
}

impl ImageSegmenterPreparing for Sam2Segmenter {
    fn prepare(&mut self, image: Image) -> Result<(), SegmentationErrors> {
        let img_data = image.image_data().image();
        let img = image::load_from_memory(img_data).map_err(|e| {
            SegmentationErrors::ImageLoadError(format!("Failed to load image: {}", e))
        })?;

        let embedding = self
            .encode_image(&img)
            .map_err(|e| SegmentationErrors::InferenceError(format!("Image encoding failed: {}", e)))?;

        self.image_embedding = Some(Arc::new(embedding));
        Ok(())
    }
}
