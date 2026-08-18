use std::{io::Cursor, sync::Arc};

use axum::{body::Body, extract::State, response::Response};
use image::RgbaImage;

use crate::{application::usecase::get_image_usecase::get_image_input::GetImageInput, composition::wiring::App, domain::value_object::image_id::image_id::ImageId, presentation::errors::{app_error::AppError, presentation_error::PresentationError}};

#[axum::debug_handler]
pub async fn get_image(
    State(app): State<Arc<App>>,
    image_id: String,
) -> Result<Response, AppError>{
    let image_id = ImageId::from_str(&image_id)
        .map_err(|err| PresentationError::from(err))?;
    let input = GetImageInput::new(image_id);
    let output = app.get_image(input)?;

    let (image, size) = output.into_image_data();
    let rgb_image = RgbaImage::from_raw(size.width() as u32, size.height() as u32, image)
        .ok_or(PresentationError::Loading)?;

    let mut bytes = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);

    rgb_image.write_to(&mut cursor, image::ImageFormat::Png).map_err(|err| PresentationError::ConvertToPng(err.to_string()))?;

    Ok(Response::builder()
        .header("Content-Type", "image/png")
        .body(Body::from(bytes))
        .unwrap())
}