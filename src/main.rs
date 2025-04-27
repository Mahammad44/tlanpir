use axum::{response::Json, routing::get, Router};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[derive(Debug, Serialize)]
struct PlaneData {
    icao24: String,
    callsign: Option<String>,
    longitude: Option<f64>,
    latitude: Option<f64>,
    altitude: Option<f64>,
    velocity: Option<f64>,
    country: String,
}

async fn get_flight_data() -> Result<Vec<PlaneData>, reqwest::Error> {
    let response = reqwest::get("https://opensky-network.org/api/states/all")
        .await?
        .json::<serde_json::Value>()
        .await?;

    let states = response["states"]
        .as_array()
        .cloned()
        .unwrap_or_default();

    let mut planes = Vec::new();

    for state in states {
        let state = state.as_array().unwrap();
        planes.push(PlaneData {
            icao24: state[0].as_str().unwrap_or_default().to_string(),
            callsign: state[1].as_str().map(|s| s.trim().to_string()),
            longitude: state[5].as_f64(),
            latitude: state[6].as_f64(),
            altitude: state[7].as_f64(),
            velocity: state[9].as_f64(),
            country: state[2].as_str().unwrap_or_default().to_string(),
        });
    }

    Ok(planes)
}

async fn flight_handler() -> Json<Vec<PlaneData>> {
    match get_flight_data().await {
        Ok(planes) => Json(planes),
        Err(_) => Json(Vec::new()),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/flights", get(flight_handler))
        .nest_service("/", ServeDir::new("static"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
