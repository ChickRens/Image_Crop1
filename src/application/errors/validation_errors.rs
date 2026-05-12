use crate::application::errors::validation::segment_input_errors::SegmentInputErrors;
use crate::application::errors::validation::session_errors::SessionErrors;

#[derive(Debug, PartialEq, Eq)]
pub enum ValidationErrors {
    Session(SessionErrors),
    Points(SegmentInputErrors),
}

impl From<SessionErrors> for ValidationErrors {
    fn from(value: SessionErrors) -> Self {
        Self::Session(value)
    }
}

impl From<SegmentInputErrors> for ValidationErrors {
    fn from(value: SegmentInputErrors) -> Self {
        Self::Points(value)
    }
}
