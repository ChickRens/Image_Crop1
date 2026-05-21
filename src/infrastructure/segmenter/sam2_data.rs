use ndarray::{Array3, Array4};

pub struct SAM2StaticContext {
    image_embeddings: Array4<f32>,
    high_res_feature_s0: Array4<f32>,
    high_res_feature_s1: Array4<f32>,
}

impl SAM2StaticContext {
    pub fn new(
        image_embeddings: Array4<f32>,
        high_res_feature_s0: Array4<f32>,
        high_res_feature_s1: Array4<f32>,
    ) -> Self {
        Self {
            image_embeddings,
            high_res_feature_s0,
            high_res_feature_s1,
        }
    }
}

pub struct SAM2InferenceContext {
    sparse_embeddings: Array3<f32>,
    dense_embeddings: Array4<f32>,
}

impl SAM2InferenceContext {
    pub fn new(sparse_embeddings: Array3<f32>, dense_embeddings: Array4<f32>) -> Self {
        Self {
            sparse_embeddings,
            dense_embeddings,
        }
    }
}
