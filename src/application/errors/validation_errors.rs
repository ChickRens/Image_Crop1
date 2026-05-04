use crate::application::errors::validation::point_errors::PointErrors;
use crate::application::errors::validation::session_errors::SessionErrors;

pub enum ValidationErrors {
    Session(SessionErrors),
    Points(PointErrors),
}

impl From<SessionErrors> for ValidationErrors {
    fn from(value: SessionErrors) -> Self {
        Self::Session(value)
    }
}

impl From<PointErrors> for ValidationErrors {
    fn from(value: PointErrors) -> Self {
        Self::Points(value)
    }
}
