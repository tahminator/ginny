use std::error::Error;

use ginny::{app, config::Config, state::AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let state = AppState::new(Config::from_env());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app(state)).await?;

    Ok(())
}
