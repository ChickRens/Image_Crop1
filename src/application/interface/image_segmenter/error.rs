use crate::leaf_detail_error;

leaf_detail_error!(
    pub enum SegmenterError {
        ModelLoadError => "MODEL_LOAD_ERROR",
        InferenceError => "INFERENCE_ERROR",
        PreProcessError => "PREPROCESS_ERROR",
    }
);
