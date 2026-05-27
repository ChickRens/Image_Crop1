use crate::domain::value_object::point::Point;
use ndarray::{Array3, Array4, ArrayBase, ArrayView3, ArrayView4, Dim, OwnedRepr};

#[derive(Debug)]
pub struct ImageEmbeddings {
    value: Array4<f32>,
}

impl ImageEmbeddings {
    pub fn new(value: Array4<f32>) -> Self {
        Self { value }
    }

    pub fn view(&self) -> ArrayView4<f32> {
        self.value.view()
    }
}

#[derive(Debug)]
pub struct HighResFeatureS0 {
    value: Array4<f32>,
}

impl HighResFeatureS0 {
    pub fn new(value: Array4<f32>) -> Self {
        Self { value }
    }

    pub fn view(&self) -> ArrayView4<f32> {
        self.value.view()
    }
}

#[derive(Debug)]
pub struct HighResFeatureS1 {
    value: Array4<f32>,
}

impl HighResFeatureS1 {
    pub fn new(value: Array4<f32>) -> Self {
        Self { value }
    }

    pub fn view(&self) -> ArrayView4<f32> {
        self.value.view()
    }
}

#[derive(Debug)]
pub struct SAM2StaticContext {
    image_embeddings: ImageEmbeddings,
    high_res_feature_s0: HighResFeatureS0,
    high_res_feature_s1: HighResFeatureS1,
}

impl SAM2StaticContext {
    pub fn new(
        image_embeddings: ImageEmbeddings,
        high_res_feature_s0: HighResFeatureS0,
        high_res_feature_s1: HighResFeatureS1,
    ) -> Self {
        Self {
            image_embeddings: image_embeddings,
            high_res_feature_s0: high_res_feature_s0,
            high_res_feature_s1: high_res_feature_s1,
        }
    }

    pub fn get_all_context_refs(&self) -> (&ImageEmbeddings, &HighResFeatureS0, &HighResFeatureS1) {
        (
            &self.image_embeddings,
            &self.high_res_feature_s0,
            &self.high_res_feature_s1,
        )
    }
}

#[derive(Debug)]
pub struct SparseEmbeddings {
    value: Array3<f32>,
}

impl SparseEmbeddings {
    pub fn new(value: Array3<f32>) -> Self {
        Self { value }
    }

    pub fn view(&self) -> ArrayView3<f32> {
        self.value.view()
    }
}

#[derive(Debug)]
pub struct DenseEmbeddings {
    value: Array4<f32>,
}

impl DenseEmbeddings {
    pub fn new(value: Array4<f32>) -> Self {
        Self { value }
    }

    pub fn view(&self) -> ArrayView4<f32> {
        self.value.view()
    }
}

#[derive(Debug)]
pub struct Mask {
    value: Array4<f32>,
}

impl Mask {
    pub fn new(value: Array4<f32>) -> Self {
        Self { value }
    }

    pub fn into_mask(self) -> ArrayBase<OwnedRepr<f32>, Dim<[usize; 4]>> {
        self.value
    }

    pub fn view(&self) -> ArrayView4<f32> {
        self.value.view()
    }
}

#[derive(Debug)]
pub struct SAM2InferenceContext {
    masks: Option<Mask>,
}

impl SAM2InferenceContext {
    pub fn new(masks: Option<Mask>) -> Self {
        Self { masks }
    }

    pub fn get_all_context_refs(&self) -> &Option<Mask> {
        &self.masks
    }
}

pub struct SAM2Inputs {
    pub points: Option<Vec<Point>>,
    pub static_context: SAM2StaticContext,
    pub inference_context: SAM2InferenceContext,
}
