#[cfg(test)]
mod errors_tests {
    use crate::application::errors::application_errors::{ApplicationErrors, Code};
    use crate::application::errors::loading_errors::LoadingErrors;
    use crate::application::errors::repository_errors::RepositoryErrors;
    use crate::application::errors::validation::segment_input_errors::SegmentInputErrors;
    use crate::application::errors::validation::session_errors::SessionErrors;
    use crate::application::errors::validation_errors::ValidationErrors;

    #[test]
    fn test_loading_errors_code() {
        assert_eq!(LoadingErrors::InvalidSize.code(), "INVALID_SIZE");
        assert_eq!(LoadingErrors::UnsupportedFormat.code(), "INVALID_FORMAT");
        assert_eq!(LoadingErrors::CorruptedImage.code(), "CORRUPTED");
    }

    #[test]
    fn test_repository_errors_code() {
        assert_eq!(RepositoryErrors::ImageNotFound.code(), "IMAGE_NOT_FOUND");
        assert_eq!(RepositoryErrors::SessionNotFound.code(), "SESSION_NOT_FOUND");
    }

    #[test]
    fn test_validation_errors_conversion() {
        let session_error = ValidationErrors::from(SessionErrors::NoSession);
        assert_eq!(session_error, ValidationErrors::Session(SessionErrors::NoSession));

        let point_error = ValidationErrors::from(SegmentInputErrors::EmptyPoint);
        assert_eq!(point_error, ValidationErrors::Points(SegmentInputErrors::EmptyPoint));
    }

    #[test]
    fn test_application_errors_from_nested_errors() {
        let loading_error: ApplicationErrors = LoadingErrors::UnsupportedFormat.into();
        assert_eq!(loading_error, ApplicationErrors::ImageLoadError(LoadingErrors::UnsupportedFormat));

        let repository_error: ApplicationErrors = RepositoryErrors::ImageNotFound.into();
        assert_eq!(repository_error, ApplicationErrors::RepositoryError(RepositoryErrors::ImageNotFound));
    }
}
