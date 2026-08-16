use std::sync::Arc;

use axum::{Router, routing::post};
use tower_http::services::ServeDir;

use crate::{composition::wiring::App, presentation::handler::upload::handler::upload};

pub fn route(app: Arc<App>) -> Router {
    Router::new()
    .route("/upload", post(upload))
    .fallback_service(ServeDir::new("frontend/pages")
    .append_index_html_on_directories(true)
    ).with_state(app)
}