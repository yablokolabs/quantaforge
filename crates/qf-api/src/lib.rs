pub mod routes;
pub mod state;

use axum::routing::{get, post};
use axum::Router;
use state::AppState;
use std::sync::Arc;

pub fn create_router() -> Router {
    let state = Arc::new(AppState::new());
    Router::new()
        .route("/health", get(routes::health::health))
        .route("/circuits", post(routes::circuits::create_circuit))
        .route("/circuits/{id}", get(routes::circuits::get_circuit))
        .route(
            "/jobs",
            post(routes::jobs::submit_job).get(routes::jobs::list_jobs),
        )
        .route("/jobs/{id}", get(routes::jobs::get_job))
        .with_state(state)
}
