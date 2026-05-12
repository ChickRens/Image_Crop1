use crate::application::errors::application_errors::Code;

#[derive(Debug, PartialEq, Eq)]
pub enum SegmentInputErrors {
    EmptyPoint,
    OutSidePoints,
    InvalidSegmentId,
}

impl Code for SegmentInputErrors {
    fn code(&self) -> &str {
        match self {
            Self::EmptyPoint => "EMPTY_POINT",
            Self::OutSidePoints => "OUT_OF_RANGE_ACCESS",
            Self::InvalidSegmentId => "INVALID_CROP_ID",
        }
    }
}
