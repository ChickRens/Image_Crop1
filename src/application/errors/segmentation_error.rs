use crate::application::errors::application_errors::Code;

#[derive(Debug, PartialEq, Eq)]
pub enum SegmentationErrors {
    RunningError,
    PreParingError,
}

impl Code for SegmentationErrors {
    fn code(&self) -> &str {
        match self {
            Self::RunningError => "RUNNING_ERROR",
            Self::PreParingError => "PREPARE_ERROR",
        }
    }
}
