use std::{sync::Arc, time::Instant};

use axum::{
    body::Body,
    extract::{Request, State},
    http::Response,
    middleware::Next,
};

use crate::{
    application::{
        interface::usage_recorder::UsageRecorder,
        types::usage::{event::UsageEvent, operation::UsageOperation, status::UsageStatus},
    },
    composition::wiring::App,
    presentation::middleware::extension::UsageStatusExt,
};

pub async fn track_usage(
    State(app): State<Arc<App>>,
    request: Request,
    next: Next,
) -> Response<Body> {
    let operation = request.uri().path().trim_start_matches("/").to_string();
    let operation = UsageOperation::new(&operation);
    let start = Instant::now();

    let response = next.run(request).await;

    let end = start.elapsed();

    let status = match response.extensions().get::<UsageStatusExt>() {
        None => UsageStatus::Success,
        Some(err) => UsageStatus::Failed(err.clone().code),
    };

    let event = UsageEvent::new(operation, status, end);
    app.usage_recorder().record(event).await;

    response
}
