use crate::application::errors::segmentation_error::SegmentationErrors;
use crate::application::interface::image_segmenter::{ImageSegmenter, ImageSegmenterPreparing};
use crate::application::types::mask::Mask as ApplicationMask;
use crate::domain::entity::image::Image;
use crate::infrastructure::segmenter::sam2_data::{
    DenseEmbeddings, HighResFeatureS0, HighResFeatureS1, ImageEmbeddings, Mask, SAM2Inputs,
    SAM2StaticContext, SparseEmbeddings,
};
use image::{DynamicImage, GenericImageView, Pixel};
use ndarray::prelude::{ArrayBase, Dim};
use ndarray::{Array, Array3, Array4, ArrayView4, Ix3, Ix4, OwnedRepr, ViewRepr};
use ort::session::builder::GraphOptimizationLevel;
use ort::session::{Session, SessionOutputs};
use ort::value::{
    DynValueTypeMarker, PrimitiveTensorElementType, TensorRef, TensorValueType, Value,
};
use std::collections::HashMap;

type OrtResult<T> = Result<T, Box<dyn std::error::Error>>;

type OrtInputs = HashMap<String, Value<DynValueTypeMarker>>;

pub struct Sam2Segmenter {
    encoder_session: Session,
    prompt_encoder_session: Session,
    mask_decoder_session: Session,
}

impl Sam2Segmenter {
    pub fn new(model_dir: &str) -> OrtResult<Self> {
        ort::init();

        Ok(Self {
            encoder_session: Self::build_session(format!(
                "{}/sam2.1_hiera_small_encoder.onnx",
                model_dir
            ))?,
            prompt_encoder_session: Self::build_session(format!(
                "{}/sam2.1_hiera_small_prompt_encoder.onnx",
                model_dir
            ))?,
            mask_decoder_session: Self::build_session(format!(
                "{}/sam2.1_hiera_small_mask_decoder.onnx",
                model_dir
            ))?,
        })
    }

    fn build_session(path: String) -> OrtResult<Session> {
        Ok(Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .commit_from_file(path)?)
    }

    fn _tensor_to_array4<OutputType>(onnx_output: &SessionOutputs, name: &str) -> Array4<OutputType>
    where
        OutputType: PrimitiveTensorElementType + Clone,
    {
        let (shape, data) = onnx_output[name]
            .try_extract_tensor::<OutputType>()
            .unwrap();

        let dims: Vec<usize> = shape.iter().map(|x| *x as usize).collect();

        Array4::from_shape_vec(Ix4(dims[0], dims[1], dims[2], dims[3]), data.to_vec()).unwrap()
    }

    fn _tensor_to_array3<OutputType>(onnx_output: &SessionOutputs, name: &str) -> Array3<OutputType>
    where
        OutputType: PrimitiveTensorElementType + Clone,
    {
        let (shape, data) = onnx_output[name]
            .try_extract_tensor::<OutputType>()
            .unwrap();

        let dims: Vec<usize> = shape.iter().map(|x| *x as usize).collect();

        Array3::from_shape_vec(Ix3(dims[0], dims[1], dims[2]), data.to_vec()).unwrap()
    }

    fn _encode_image(
        &mut self,
        original_image: &DynamicImage,
    ) -> ort::Result<(ImageEmbeddings, HighResFeatureS0, HighResFeatureS1)> {
        let resized_image =
            original_image.resize_exact(1024, 1024, image::imageops::FilterType::CatmullRom);

        let mut input: ArrayBase<OwnedRepr<f32>, Dim<[usize; 4]>, f32> =
            Array::zeros((1, 3, 1024, 1024));
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
        let outputs: &SessionOutputs = &self
            .encoder_session
            .run(ort::inputs!["image" => input_value])?;

        // 出力を取得
        let image_embeddings_value: ArrayBase<OwnedRepr<f32>, Dim<[usize; 4]>, f32> =
            Self::_tensor_to_array4::<f32>(outputs, "image_embeddings");
        let high_res_feature_s0_value: ArrayBase<OwnedRepr<f32>, Dim<[usize; 4]>, f32> =
            Self::_tensor_to_array4::<f32>(outputs, "high_res_feature_s0");
        let high_res_feature_s1_value: ArrayBase<OwnedRepr<f32>, Dim<[usize; 4]>, f32> =
            Self::_tensor_to_array4::<f32>(outputs, "high_res_feature_s1");

        let image_embeddings = ImageEmbeddings::new(image_embeddings_value);
        let high_res_feature_s0 = HighResFeatureS0::new(high_res_feature_s0_value);
        let high_res_feature_s1 = HighResFeatureS1::new(high_res_feature_s1_value);

        Ok((image_embeddings, high_res_feature_s0, high_res_feature_s1))
    }

    fn _encode_prompt(
        &mut self,
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

        let mut input_coords: ArrayBase<OwnedRepr<f32>, Dim<[usize; 3]>, f32> =
            Array::zeros((1, points_coords.len(), 2));
        for (i, coord) in points_coords.iter().enumerate() {
            input_coords[[0, i, 0]] = coord.0;
            input_coords[[0, i, 1]] = coord.1;
        }

        let mut input_labels: ArrayBase<OwnedRepr<i64>, Dim<[usize; 2]>, i64> =
            Array::zeros((1, points_labels.len()));
        for (i, label) in points_labels.iter().enumerate() {
            input_labels[[0, i]] = *label;
        }

        let none_binding: ArrayBase<OwnedRepr<f32>, Dim<[usize; 4]>, f32> =
            Array::zeros((1, 1, 256, 256));
        let input_mask: ArrayBase<ViewRepr<&f32>, Dim<[usize; 4]>, f32> = match mask {
            Some(mask) => mask,
            None => none_binding.view(),
        };

        let outputs = &self.encoder_session.run(ort::inputs![
                "points_coords" => TensorRef::from_array_view(&input_coords)?,
                "points_labels" => TensorRef::from_array_view(&input_labels)?,
                "mask"          => TensorRef::from_array_view(input_mask)?])?;

        let sparse_embeddings_value: ArrayBase<OwnedRepr<f32>, Dim<[usize; 3]>, f32> =
            Self::_tensor_to_array3::<f32>(outputs, "sparse_embeddings");
        let dense_embeddings_value: ArrayBase<OwnedRepr<f32>, Dim<[usize; 4]>, f32> =
            Self::_tensor_to_array4::<f32>(outputs, "dense_embeddings");

        let sparse_embeddings = SparseEmbeddings::new(sparse_embeddings_value);
        let dense_embeddings = DenseEmbeddings::new(dense_embeddings_value);

        Ok((sparse_embeddings, dense_embeddings))
    }

    fn _decode_mask(
        &mut self,
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

        let input_image_embeddings: ArrayBase<ViewRepr<&f32>, Dim<[usize; 4]>, f32> =
            image_embeddings.view();
        let input_sparse_embeddings: ArrayBase<ViewRepr<&f32>, Dim<[usize; 3]>, f32> =
            sparse_embeddings.view();
        let input_dense_embeddings: ArrayBase<ViewRepr<&f32>, Dim<[usize; 4]>, f32> =
            dense_embeddings.view();
        let input_high_res_feature_s0: ArrayBase<ViewRepr<&f32>, Dim<[usize; 4]>, f32> =
            high_res_feature_s0.view();
        let input_high_res_feature_s1: ArrayBase<ViewRepr<&f32>, Dim<[usize; 4]>, f32> =
            high_res_feature_s1.view();

        let outputs: &SessionOutputs = &self.mask_decoder_session.run(ort::inputs![
            "image_embeddings" => TensorRef::from_array_view(input_image_embeddings)?,
            "sparse_prompt_embeddings" => TensorRef::from_array_view(input_sparse_embeddings)?,
            "dense_prompt_embeddings" => TensorRef::from_array_view(input_dense_embeddings)?,
            "high_res_feature_s0" => TensorRef::from_array_view(input_high_res_feature_s0)?,
            "high_res_feature_s1" => TensorRef::from_array_view(input_high_res_feature_s1)?])?;

        let mask_value: ArrayBase<OwnedRepr<f32>, Dim<[usize; 4]>, f32> =
            Self::_tensor_to_array4::<f32>(outputs, "masks");

        let mask = Mask::new(mask_value);
        Ok(mask)
    }
}

impl ImageSegmenter for Sam2Segmenter {
    type SegmentationInputs = SAM2Inputs;

    fn segment(
        &mut self,
        request: Self::SegmentationInputs,
    ) -> Result<ApplicationMask, SegmentationErrors> {
        let static_context = request.static_context;
        let inference_context = request.inference_context;

        let (image_emb, s0, s1) = static_context.get_all_context_refs();
        let mask = inference_context.get_all_context_refs();

        let mut coords: Vec<(f32, f32)> = Vec::new();
        let mut labels: Vec<i64> = Vec::new();
        let mut mask_value: Option<ArrayView4<f32>> = None;

        if let Some(points) = request.points {
            for point in points {
                let point_x = point.coordinate().x() as f32;
                let point_y = point.coordinate().y() as f32;
                let label = point.label() as i64;
                coords.push((point_x, point_y));
                labels.push(label);
            }
        }

        if let Some(mask) = mask {
            mask_value = Some(mask.view())
        }

        let (sparse_emb, dense_emb) =
            self._encode_prompt(coords, labels, mask_value)
                .map_err(|e| {
                    SegmentationErrors::InferenceError(format!("Prompt encoding failed: {}", e))
                })?;

        let mask = self
            ._decode_mask(&image_emb, &sparse_emb, &dense_emb, s0, s1)
            .map_err(|e| {
                SegmentationErrors::InferenceError(format!("Mask decoding failed: {}", e))
            })?;

        Ok(ApplicationMask::new(mask.into_mask()))
    }
}

impl ImageSegmenterPreparing for Sam2Segmenter {
    type SegmentationContext = SAM2StaticContext;

    fn prepare(&mut self, image: Image) -> Result<Self::SegmentationContext, SegmentationErrors> {
        let img_data = image.image_data().image();
        let img = image::load_from_memory(img_data).map_err(|e| {
            SegmentationErrors::ImageLoadError(format!("Failed to load image: {}", e))
        })?;

        let (embedding, s0, s1) = self._encode_image(&img).map_err(|e| {
            SegmentationErrors::InferenceError(format!("Image encoding failed: {}", e))
        })?;

        let context = SAM2StaticContext::new(embedding, s0, s1);
        Ok(context)
    }
}
