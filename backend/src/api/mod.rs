use axum::{Router};
use axum::routing::get;
use crate::db::{DbPool};

mod stops;
mod quays;

pub async fn api_router(pool: DbPool) -> Router {
    Router::new()
        .route("/stops", get(stops::list))
        .route("/stops/:id", get(stops::show))
        .route("/quays/:id", get(quays::show))
        .with_state(pool)
    //
    // .with_state(AppState {
    //     pool: create_db_pool().await
    // })
}

