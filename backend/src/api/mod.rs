use axum::{Router};
use axum::routing::get;
use crate::db::{DbPool};

mod stops;
mod quays;
mod journeys;
mod favorites;

pub async fn api_router(pool: DbPool) -> Router {
    Router::new()
        .route("/stops", get(stops::list))
        .route("/stops/:id", get(stops::show))
        .route("/quays/:id", get(quays::show))
        .route("/journeys", get(journeys::list))
        .route("/favorites", get(favorites::list))
        .route("/journeys/:id", get(journeys::show))
        .with_state(pool)
}

