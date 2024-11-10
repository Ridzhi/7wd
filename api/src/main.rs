mod state;
mod app;
mod account;
mod prelude;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let state = Arc::new(state::AppState::default());
    let _ = state.pg().get().await.expect("failed to get pg connection");
    let _ = state.rds().get_multiplexed_async_connection().await.expect("failed to get redis connection");

    let config = state.config().clone();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/account", account::handler::build(state.clone()))
        ;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
