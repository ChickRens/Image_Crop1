use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    application::error::ApplicationError, common::traits::{Cause, Code, ErrorType, ErrorTypeProvider}, parent_error, presentation::errors::presentation_error::PresentationError,
};

parent_error!(
    pub enum AppError {
        ApplicationError(ApplicationError),
        Presentation(PresentationError),
    }
);

#[derive(serde::Serialize)]
struct ErrorBody {
    code: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self.error_type() {
            ErrorType::NotFound => StatusCode::NOT_FOUND,
            ErrorType::Conflict => StatusCode::CONFLICT,
            ErrorType::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::InvalidInput => StatusCode::BAD_REQUEST,
        };

        let code = self.code();

        eprintln!("AppError: {:?}", self.cause());

        let json = Json(ErrorBody {
            code: code.to_string(),
        });

        (status, json).into_response()
    }
}
