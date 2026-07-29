use crate::application::errors::validation::segment_input_errors::SegmentInputErrors;
use crate::application::errors::validation::session_errors::SessionErrors;
use crate::domain::errors::traits::{Cause, Code};

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

impl Code for ValidationErrors {
    fn code(&self) -> &str {
        match self {
            Self::Points(err) => err.code(),
            Self::Session(err) => err.code(),
        }
    }
}

impl Cause for ValidationErrors {
    fn cause(&self) -> &str {
        match self {
            Self::Points(err) => err.cause(),
            Self::Session(err) => err.cause()
        }
    }
}