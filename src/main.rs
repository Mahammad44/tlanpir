mod server;

use axum::{Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Setup the server using the server module
    let app = server::create_app();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

