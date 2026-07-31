use crate::domain::errors::traits::Code;

#[derive(Debug, PartialEq, Eq)]
pub enum SegmentationErrors {
    InferenceError(String),
    InitializeError(String),
}

impl Code for SegmentationErrors {
    fn code(&self) -> &str {
        match self {
            Self::InferenceError(_) => "INFERENCE_ERROR",
            Self::InitializeError(_) => "INITIALIZE_ERROR",
        }
    }
}
