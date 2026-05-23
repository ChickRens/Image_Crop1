use std::rc::Rc;

use crate::domain::value_object::point::Point;
use ndarray::{Array3, Array4};

#[derive(Debug, Clone)]
pub struct SAM2StaticContext {
    image_embeddings: Rc<Array4<f32>>,
    high_res_feature_s0: Rc<Array4<f32>>,
    high_res_feature_s1: Rc<Array4<f32>>,
}

impl SAM2StaticContext {
    pub fn new(
        image_embeddings: Array4<f32>,
        high_res_feature_s0: Array4<f32>,
        high_res_feature_s1: Array4<f32>,
    ) -> Self {
        Self {
            image_embeddings: image_embeddings.into(),
            high_res_feature_s0: high_res_feature_s0.into(),
            high_res_feature_s1: high_res_feature_s1.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SAM2InferenceContext {
    sparse_embeddings: Rc<Array3<f32>>,
    dense_embeddings: Rc<Array4<f32>>,
    masks: Rc<Array4<f32>>,
}

impl SAM2InferenceContext {
    pub fn new(
        sparse_embeddings: Array3<f32>,
        dense_embeddings: Array4<f32>,
        masks: Array4<f32>,
    ) -> Self {
        Self {
            sparse_embeddings: sparse_embeddings.into(),
            dense_embeddings: dense_embeddings.into(),
            masks: masks.into(),
        }
    }
}

pub struct SAM2Inputs {
    pub points: Option<Vec<Point>>,
}
