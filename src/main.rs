use std::error::Error;

use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse {
    version: String,
}

async fn api() -> Json<ApiResponse> {
    let version = std::env::var("VERSION").unwrap_or_else(|_| "N/A".to_string());
    Json(ApiResponse { version })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let app = Router::new().route("/api", get(api));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
