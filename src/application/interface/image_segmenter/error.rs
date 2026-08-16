use crate::{common::traits::ErrorType::Internal, leaf_detail_error};

leaf_detail_error!(
    pub enum SegmenterError {
        ModelLoadError => ("MODEL_LOAD_ERROR", Internal),
        InferenceError => ("INFERENCE_ERROR", Internal),
        PreProcessError => ("PREPROCESS_ERROR", Internal),
    }
);
