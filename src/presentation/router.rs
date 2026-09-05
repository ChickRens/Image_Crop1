use std::sync::Arc;

use axum::{Router, routing::post};
use tower_http::services::ServeDir;

use crate::{
    composition::wiring::App,
    presentation::handler::{
        get_image::handler::get_image, redo::handler::redo, segment::handler::segment,
        undo::handler::undo, upload::handler::upload,
    },
};

pub fn route(app: Arc<App>) -> Router {
    Router::new()
        .route("/upload", post(upload))
        .route("/get-image", post(get_image))
        .route("/segment", post(segment))
        .route("/undo", post(undo))
        .route("/redo", post(redo))
        .fallback_service(ServeDir::new("frontend").append_index_html_on_directories(true))
        .with_state(app)
}
