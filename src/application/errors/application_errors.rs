use crate::application::errors::loading_errors::LoadingErrors;
use crate::application::errors::repository_errors::RepositoryErrors;
use crate::application::errors::validation_errors::ValidationErrors;

pub trait Code {
    fn code(&self) -> &str;
}

pub enum ApplicationErrors {
    ImageLoadError(LoadingErrors),
    ValidationError(ValidationErrors),
    SegmentationError(),
    RepositoryError(RepositoryErrors),
}

impl From<LoadingErrors> for ApplicationErrors {
    fn from(value: LoadingErrors) -> Self {
        Self::ImageLoadError(value)
    }
}

impl From<ValidationErrors> for ApplicationErrors {
    fn from(value: ValidationErrors) -> Self {
        Self::ValidationError(value)
    }
}

impl From<RepositoryErrors> for ApplicationErrors {
    fn from(value: RepositoryErrors) -> Self {
        Self::RepositoryError(value)
    }
}

