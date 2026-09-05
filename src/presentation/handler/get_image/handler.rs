use std::sync::Arc;

use axum::{Json, body::Body, extract::State, response::Response};

use crate::{
    application::usecase::get_image_usecase::get_image_input::GetImageInput,
    composition::wiring::App,
    domain::value_object::image_id::image_id::ImageId,
    presentation::{
        errors::{app_error::AppError, presentation_error::PresentationError},
        handler::get_image::request::GetImageRequest,
    },
};

#[axum::debug_handler]
pub async fn get_image(
    State(app): State<Arc<App>>,
    Json(request): Json<GetImageRequest>,
) -> Result<Response, AppError> {
    println!("Getting image start");
    let image_id = request.image_id;
    let image_id = ImageId::from_str(&image_id).map_err(|err| PresentationError::from(err))?;

    let input = GetImageInput::new(image_id);

    let output = app.get_image(input)?;

    let (image, _) = output.image_data();

    Ok(Response::builder()
        .header("Content-Type", "image/webp")
        .body(Body::from(image))
        .unwrap())
}
