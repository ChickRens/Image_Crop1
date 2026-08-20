use std::sync::Mutex;

use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel};
use ndarray::{Array2, Array3, Array4, ArrayView3, ArrayView4, Ix3, Ix4};
use ort::{
    session::{Session, SessionOutputs, builder::GraphOptimizationLevel},
    value::{PrimitiveTensorElementType, TensorRef, TensorValueType, Value},
};

use crate::{
    application::{
        interface::image_segmenter::{error::SegmenterError, segmenter::ImageSegmenter},
        types::segmented_image::SegmentedImage,
    },
    domain::{
        entity::image::image::Image,
        value_object::{image_data::ImageData, point::Point},
    },
    infrastructure::segmenter::{
        mask_applier::SAM2MaskApplier,
        mask_resizer::SAM2MaskResizer,
        sam2_data::{
            DenseEmbeddings, HighResFeatureS0, HighResFeatureS1, ImageEmbeddings, Mask,
            SAM2InferenceContext, SAM2StaticContext, SparseEmbeddings,
        },
    },
};
use std::fs::read;

pub struct Sam2Segmenter {
    image_encoder_session: Mutex<Session>,
    prompt_encoder_session: Mutex<Session>,
    mask_decoder_session: Mutex<Session>,
    image_pe: Array4<f32>,
}

impl Sam2Segmenter {
    pub fn new(model_dir: &str) -> Result<Self, SegmenterError> {
        Ok(Self {
            image_encoder_session: Mutex::new(Self::build_session(format!(
                "{}/sam2.1_hiera_small_image_encoder.onnx",
                model_dir
            ))?),
            prompt_encoder_session: Mutex::new(Self::build_session(format!(
                "{}/sam2.1_hiera_small_prompt_encoder.onnx",
                model_dir
            ))?),
            mask_decoder_session: Mutex::new(Self::build_session(format!(
                "{}/sam2.1_hiera_small_mask_decoder.onnx",
                model_dir
            ))?),
            image_pe: {
                let bin_file = read("models/image_pe.bin")
                    .map_err(|e| SegmenterError::ModelLoadError(e.to_string()))?;
                let data: &[f32] = bytemuck::cast_slice(&bin_file);
                Array4::from_shape_vec((1, 256, 64, 64), data.to_vec())
                    .map_err(|e| SegmenterError::ModelLoadError(e.to_string()))?
            },
        })
    }

    fn build_session(path: String) -> Result<Session, SegmenterError> {
        Ok(Session::builder()
            .map_err(|e| SegmenterError::ModelLoadError(e.to_string()))?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(|e| SegmenterError::ModelLoadError(e.to_string()))?
            .commit_from_file(path)
            .map_err(|e| SegmenterError::ModelLoadError(e.to_string())))?
    }

    fn _tensor_to_array4<OutputType>(onnx_output: &SessionOutputs, name: &str) -> Array4<OutputType>
    where
        OutputType: PrimitiveTensorElementType + Clone,
    {
        let (shape, data) = onnx_output[name]
            .try_extract_tensor::<OutputType>()
            .expect("Failed to extract tensor from ONNX output");

        let dims: Vec<usize> = shape.iter().map(|x| *x as usize).collect();

        Array4::from_shape_vec(Ix4(dims[0], dims[1], dims[2], dims[3]), data.to_vec())
            .expect("Failed to create Array4 from tensor data")
    }

    fn _tensor_to_array3<OutputType>(onnx_output: &SessionOutputs, name: &str) -> Array3<OutputType>
    where
        OutputType: PrimitiveTensorElementType + Clone,
    {
        let (shape, data) = onnx_output[name]
            .try_extract_tensor::<OutputType>()
            .expect("Failed to extract tensor from ONNX output");

        let dims: Vec<usize> = shape.iter().map(|x| *x as usize).collect();

        Array3::from_shape_vec(Ix3(dims[0], dims[1], dims[2]), data.to_vec())
            .expect("Failed to create Array3 from tensor data")
    }

    fn _encode_image(
        &self,
        original_image: &DynamicImage,
    ) -> ort::Result<(ImageEmbeddings, HighResFeatureS0, HighResFeatureS1)> {
        let mut session = self
            .image_encoder_session
            .lock()
            .expect("ImageEncoderSession Mutex is Poisoned");

        let resized_image =
            original_image.resize_exact(1024, 1024, image::imageops::FilterType::CatmullRom);

        let mut input: Array4<f32> = Array4::zeros((1, 3, 1024, 1024));
        for pixel in resized_image.pixels() {
            let x = pixel.0 as usize;
            let y = pixel.1 as usize;
            let channels = pixel.2.channels();
            let r: u8 = channels[0];
            let g: u8 = channels[1];
            let b: u8 = channels[2];

            input[[0, 0, y, x]] = (r as f32) / 255.0;
            input[[0, 1, y, x]] = (g as f32) / 255.0;
            input[[0, 2, y, x]] = (b as f32) / 255.0;
        }

        let input_value: Value<TensorValueType<f32>> = Value::from_array(input)?;
        let outputs = session.run(ort::inputs!["image" => input_value])?;

        // 出力を取得
        let image_embeddings_value: Array4<f32> =
            Self::_tensor_to_array4::<f32>(&outputs, "image_embeddings");
        let high_res_feature_s0_value: Array4<f32> =
            Self::_tensor_to_array4::<f32>(&outputs, "high_res_feature_s0");
        let high_res_feature_s1_value: Array4<f32> =
            Self::_tensor_to_array4::<f32>(&outputs, "high_res_feature_s1");

        let image_embeddings = ImageEmbeddings::new(image_embeddings_value);
        let high_res_feature_s0 = HighResFeatureS0::new(high_res_feature_s0_value);
        let high_res_feature_s1 = HighResFeatureS1::new(high_res_feature_s1_value);

        Ok((image_embeddings, high_res_feature_s0, high_res_feature_s1))
    }

    fn _scale_prompt(
        points_coords: Vec<(f32, f32)>,
        current_height: u16,
        current_width: u16,
        target_height: u16,
        target_width: u16,
    ) -> Vec<(f32, f32)> {
        let height_scale = target_height as f64 / current_height as f64;
        let width_scale = target_width as f64 / current_width as f64;

        let mut scaled_points_coords: Vec<(f32, f32)> = Vec::with_capacity(points_coords.len());

        for coords in points_coords {
            let x = coords.0 as f64 * width_scale;
            let y = coords.1 as f64 * height_scale;

            scaled_points_coords.push((x as f32, y as f32));
        }
        scaled_points_coords
    }

    fn _encode_prompt(
        &self,
        points_coords: Vec<(f32, f32)>,
        points_labels: Vec<i64>,
        mask: Option<ArrayView4<f32>>,
    ) -> ort::Result<(SparseEmbeddings, DenseEmbeddings)> {
        // name: points_coords
        // tensor: float32[1,-1,2]
        // points_labels
        // name: points_labels
        // tensor: int64[1,-1]
        // masks
        // name: masks
        // tensor: float32[1,1,256,256]

        let mut session = self
            .prompt_encoder_session
            .lock()
            .expect("PromptEncoderSession Mutex is Poisoned");

        let mut input_coords: Array3<f32> = Array3::zeros((1, points_coords.len(), 2));
        for (i, coord) in points_coords.iter().enumerate() {
            input_coords[[0, i, 0]] = coord.0;
            input_coords[[0, i, 1]] = coord.1;
        }

        let mut input_labels: Array2<i64> = Array2::zeros((1, points_labels.len()));
        for (i, label) in points_labels.iter().enumerate() {
            input_labels[[0, i]] = *label;
        }

        let none_binding: Array4<f32> = Array4::zeros((1, 1, 256, 256));
        let input_mask: ArrayView4<f32> = match mask {
            Some(mask) => mask,
            None => none_binding.view(),
        };

        let outputs = session.run(ort::inputs![
                "point_coords" => TensorRef::from_array_view(&input_coords)?,
                "point_labels" => TensorRef::from_array_view(&input_labels)?,
                "mask"          => TensorRef::from_array_view(input_mask)?])?;

        let sparse_embeddings_value: Array3<f32> =
            Self::_tensor_to_array3::<f32>(&outputs, "sparse_embeddings");
        let dense_embeddings_value: Array4<f32> =
            Self::_tensor_to_array4::<f32>(&outputs, "dense_embeddings");

        let sparse_embeddings = SparseEmbeddings::new(sparse_embeddings_value);
        let dense_embeddings = DenseEmbeddings::new(dense_embeddings_value);

        Ok((sparse_embeddings, dense_embeddings))
    }

    fn _decode_mask(
        &self,
        image_embeddings: &ImageEmbeddings,
        sparse_embeddings: &SparseEmbeddings,
        dense_embeddings: &DenseEmbeddings,
        high_res_feature_s0: &HighResFeatureS0,
        high_res_feature_s1: &HighResFeatureS1,
    ) -> ort::Result<Mask> {
        // Input

        // image_embeddings
        // name: image_embeddings
        // tensor: float32[1,256,64,64]

        // image_pe
        // name: image_pe
        // tensor: float32[1,256,64,64]

        // sparse_prompt_embeddings
        // name: sparse_prompt_embeddings
        // tensor: float32[1,4,256]

        // dense_prompt_embeddings
        // name: dense_prompt_embeddings
        // tensor: float32[1,256,64,64]

        // high_res_feature_s0
        // name: high_res_feature_s0
        // tensor: float32[1,32,256,256]

        // high_res_feature_s1
        // name: high_res_feature_s1
        // tensor: float32[1,64,128,128]

        // Output

        // masks
        // name: masks
        // tensor: float32[1,1,256,256]

        // iou_pred
        // name: iou_pred
        // tensor: float32[1,1]

        // sam_tokens_out
        // name: sam_tokens_out
        // tensor: float32[1,1,256]

        // object_score_logits
        // name: object_score_logits
        // tensor: float32[1,1]

        let mut session = self
            .mask_decoder_session
            .lock()
            .expect("MaskDecoderSession Mutex is Poisoned");

        let input_image_embeddings: ArrayView4<f32> = image_embeddings.view();
        let input_sparse_embeddings: ArrayView3<f32> = sparse_embeddings.view();
        let input_dense_embeddings: ArrayView4<f32> = dense_embeddings.view();
        let input_high_res_feature_s0: ArrayView4<f32> = high_res_feature_s0.view();
        let input_high_res_feature_s1: ArrayView4<f32> = high_res_feature_s1.view();

        let outputs = session.run(ort::inputs![
            "image_embeddings" => TensorRef::from_array_view(input_image_embeddings)?,
            "image_pe" => TensorRef::from_array_view(self.image_pe.view())?,
            "sparse_prompt_embeddings" => TensorRef::from_array_view(input_sparse_embeddings)?,
            "dense_prompt_embeddings" => TensorRef::from_array_view(input_dense_embeddings)?,
            "high_res_feature_s0" => TensorRef::from_array_view(input_high_res_feature_s0)?,
            "high_res_feature_s1" => TensorRef::from_array_view(input_high_res_feature_s1)?])?;

        let mask_value: Array4<f32> = Self::_tensor_to_array4::<f32>(&outputs, "masks");

        let mask = Mask::new(mask_value);
        Ok(mask)
    }

    fn _inference(
        &self,
        original_image: &Image,
        static_context: &SAM2StaticContext,
        inference_context: &SAM2InferenceContext,
        input_points: &[Point],
    ) -> Result<Mask, SegmenterError> {
        let sam2_required_height = 1024;
        let sam2_required_width = 1024;

        let (image_emb, s0, s1) = static_context.get_all_context_refs();
        let mask = inference_context.get_all_context_refs();

        let mut coords: Vec<(f32, f32)> = Vec::new();
        let mut labels: Vec<i64> = Vec::new();
        let mut mask_value: Option<ArrayView4<f32>> = None;

        for point in input_points {
            let point_x = point.coordinate().x() as f32;
            let point_y = point.coordinate().y() as f32;
            let label = point.label() as i64;
            coords.push((point_x, point_y));
            labels.push(label);
        }

        if let Some(mask) = mask {
            mask_value = Some(mask.view())
        }

        let scaled_coords = Self::_scale_prompt(
            coords,
            original_image.image_size().height(),
            original_image.image_size().width(),
            sam2_required_height,
            sam2_required_width,
        );

        let (sparse_emb, dense_emb) = self
            ._encode_prompt(scaled_coords, labels, mask_value)
            .map_err(|e| {
                SegmenterError::InferenceError(format!("Prompt encoding failed: {}", e))
            })?;

        let mask = self
            ._decode_mask(&image_emb, &sparse_emb, &dense_emb, s0, s1)
            .map_err(|e| SegmenterError::InferenceError(format!("Mask decoding failed: {}", e)))?;

        Ok(mask)
    }

    fn _generate_image(
        mask: &Mask,
        original_image: &Image,
    ) -> Result<SegmentedImage, SegmenterError> {
        let resized_mask = SAM2MaskResizer::resize_mask(
            &mask.view().to_owned(),
            original_image.image_size().height() as usize,
            original_image.image_size().width() as usize,
        );

        let applied_image = SAM2MaskApplier::apply(&original_image, resized_mask.view().to_owned());

        Ok(SegmentedImage::new(
            ImageData::new(applied_image),
            original_image.image_size().clone(),
        ))
    }
}

impl ImageSegmenter for Sam2Segmenter {
    type InferenceContext = SAM2InferenceContext;
    type StaticContext = SAM2StaticContext;

    fn segment(
        &self,
        original_image: &Image,
        static_context: &Self::StaticContext,
        inference_context: &Self::InferenceContext,
        input_points: &[Point],
    ) -> Result<(Self::InferenceContext, SegmentedImage), SegmenterError> {
        let mask = self._inference(
            original_image,
            static_context,
            inference_context,
            input_points,
        )?;

        let segmented = Self::_generate_image(&mask, original_image)?;

        let inference_context = SAM2InferenceContext::new(Some(mask));

        // Ok(SegmentedImage::new(
        //     ImageData::new(applied_image),
        //     original_image.image_size().clone(),
        // ))

        Ok((inference_context, segmented))
    }

    fn rebuild(
        &self,
        original_image: &Image,
        static_context: &Self::StaticContext,
        input_points: &[Point],
    ) -> Result<(Self::InferenceContext, SegmentedImage), SegmenterError> {
        let mut inference_context = SAM2InferenceContext::new(None);

        for idx in 1..=input_points.len() - 1 {
            let current_points = &input_points[..idx];

            let temp_mask = self._inference(
                original_image,
                static_context,
                &inference_context,
                current_points,
            )?;

            inference_context = SAM2InferenceContext::new(Some(temp_mask));
        }

        let current_points = input_points;

        let mask = self._inference(
            original_image,
            static_context,
            &inference_context,
            current_points,
        )?;

        let segmented_image = Self::_generate_image(&mask, original_image)?;

        inference_context = SAM2InferenceContext::new(Some(mask));

        Ok((inference_context, segmented_image))
    }

    fn prepare_static_context(&self, image: &Image) -> Result<Self::StaticContext, SegmenterError> {
        let img_data = image.image_data().image();
        let rgba_image = ImageBuffer::from_raw(
            image.image_size().width() as u32,
            image.image_size().height() as u32,
            img_data.clone(),
        )
        .ok_or(SegmenterError::PreProcessError(
            "It is not Raw RGB data".to_string(),
        ))?;

        let img = DynamicImage::ImageRgba8(rgba_image);

        let (embedding, s0, s1) = self
            ._encode_image(&img)
            .map_err(|e| SegmenterError::InferenceError(format!("Image encoding failed: {}", e)))?;

        let static_context = SAM2StaticContext::new(embedding, s0, s1);
        Ok(static_context)
    }

    fn prepare_inference_context(
        &self,
        _image: &Image,
    ) -> Result<Self::InferenceContext, SegmenterError> {
        let inference_context = SAM2InferenceContext::new(None);
        Ok(inference_context)
    }
}
