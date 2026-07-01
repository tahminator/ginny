pub mod config;
pub mod routes;
pub mod state;

use axum::Router;

use crate::state::AppState;

pub fn app(state: AppState) -> Router {
    routes::router().with_state(state)
}
