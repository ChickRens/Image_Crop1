use crate::application::errors::application_errors::Code;

pub enum PointErrors {
    EmptyPoint,
    OutSidePoints,
}

impl Code for PointErrors {
    fn code(&self) -> &str {
        match self {
            Self::EmptyPoint => "EMPTY_POINT",
            Self::OutSidePoints => "OUT_OF_RANGE_ACCESS",
        }
    }
}
