mod application;
mod common;
mod composition;
mod domain;
mod infrastructure;
mod presentation;
mod test;

// use std::sync::Arc;

// use presentation::router;
// use tokio::net::TcpListener;

// use crate::composition::wiring::App;

#[tokio::main]
async fn main() {
    // let app = Arc::new(App::new().unwrap());
    // let router = router::route(app);
    // println!("Server running on http://localhost:3000");

    // let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, router).await.unwrap();
}
