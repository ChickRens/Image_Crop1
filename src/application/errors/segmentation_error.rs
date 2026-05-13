use crate::application::errors::application_errors::Code;

#[derive(Debug, PartialEq, Eq)]
pub enum SegmentationErrors {
    RunningError,
    PreParingError,
    NotPrepared,
    InferenceError(String),
    ImageLoadError(String),
}

impl Code for SegmentationErrors {
    fn code(&self) -> &str {
        match self {
            Self::RunningError => "RUNNING_ERROR",
            Self::PreParingError => "PREPARE_ERROR",
            Self::NotPrepared => "NOT_PREPARED",
            Self::InferenceError(_) => "INFERENCE_ERROR",
            Self::ImageLoadError(_) => "IMAGE_LOAD_ERROR",
        }
    }
}
