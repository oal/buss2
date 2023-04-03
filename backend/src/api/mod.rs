use std::collections::HashMap;
use axum::body::{Body, HttpBody};
use axum::{Error, Json, Router};
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::get;
use diesel::prelude::*;
use crate::db::{create_db_pool, DbPool};
use crate::models::{Quay, Stop};

mod stops;

pub async fn api_router(pool: DbPool) -> Router {
    Router::new()
        .route("/stops", get(stops::list))
        .route("/stops/:id", get(stops::show))
        .with_state(pool)
        //
        // .with_state(AppState {
        //     pool: create_db_pool().await
        // })
}

