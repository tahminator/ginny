use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
struct ApiResponse {
    version: Arc<str>,
}

async fn api(State(state): State<AppState>) -> Json<ApiResponse> {
    Json(ApiResponse {
        version: state.config.version.clone(),
    })
}

pub fn router() -> Router<AppState> {
    Router::new().route("/api", get(api))
}
