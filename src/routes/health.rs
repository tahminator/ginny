use axum::{Router, routing::get};

use crate::state::AppState;

async fn health() -> &'static str {
    "OK"
}

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health))
}
