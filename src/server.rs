use axum::{Router, routing::get};
use std::net::SocketAddr;

async fn plane_tracker() -> &'static str {
    "This is the Plane Tracker panel!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/plane", get(plane_tracker));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

