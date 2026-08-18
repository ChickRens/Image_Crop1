use crate::{common::traits::{Cause, Code, ErrorType::{self, Internal, InvalidInput}, ErrorTypeProvider}, domain::value_object::{image_id::error::ImageIdError, session_id::error::SessionIdError}};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PresentationError {
    Multipart(String),
    UnknownName(String),
    ConvertToPng(String),
    SessionId(SessionIdError),
    ImageId(ImageIdError),
    Loading,
    NoName,
}

impl Code for PresentationError {
    fn code(&self) -> &str {
        match self {
            Self::Multipart(_) => "MULTIPART_ERROR",
            Self::UnknownName(_) => "UNKNOWN_TAG_NAME",
            Self::ConvertToPng(_) => "CONVERT_TO_PNG_ERROR",
            Self::ImageId(err) => err.code(),
            Self::SessionId(err) => err.code(),
            Self::Loading => "LOADING_ERROR",
            Self::NoName => "NO_TAG_NAME_ERROR",
        }
    }
}

impl Cause for PresentationError {
    fn cause(&self) -> Option<&str> {
        match self {
            Self::Multipart(cause) => Some(cause),
            Self::UnknownName(cause) => Some(cause),
            Self::ConvertToPng(cause) => Some(cause),
            Self::Loading => None,
            Self::ImageId(err) => err.cause(),
            Self::SessionId(err) => err.cause(),
            Self::NoName => None,
        }
    }
}

impl From<ImageIdError> for PresentationError {
    fn from(value: ImageIdError) -> Self {
        Self::ImageId(value)
    }
}

impl From<SessionIdError> for PresentationError {
    fn from(value: SessionIdError) -> Self {
        Self::SessionId(value)
    }
}

impl ErrorTypeProvider for PresentationError {
    fn error_type(&self) -> ErrorType {
        match self {
            Self::Multipart(_) => Internal,
            Self::UnknownName(_) => InvalidInput,
            Self::ConvertToPng(_) => Internal,
            Self::Loading => Internal,
            Self::ImageId(err) => err.error_type(),
            Self::SessionId(err) => err.error_type(),
            Self::NoName => InvalidInput,
        }
    }
}