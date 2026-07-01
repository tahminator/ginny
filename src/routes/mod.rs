mod api;
mod health;

use axum::Router;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().merge(api::router()).merge(health::router())
}
