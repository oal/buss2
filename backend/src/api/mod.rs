use std::collections::HashMap;
use axum::body::{Body, HttpBody};
use axum::{Error, Json, Router};
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::get;
use diesel::prelude::*;
use crate::db::establish_connection;
use crate::models::{Quay, Stop};

mod stops;

pub fn api_router() -> Router {
    Router::new()
        .route("/stops", get(stops::list))
        .route("/stops/:id", get(stops::show))
}

