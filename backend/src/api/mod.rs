use std::collections::HashMap;
use axum::body::{Body, HttpBody};
use axum::{Error, Json, Router};
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use diesel::prelude::*;
use crate::db::establish_connection;
use crate::models::{Stop};

pub fn api_router() -> Router {
    Router::new()
        .route("/stops", get(stops))
}

#[derive(serde::Deserialize)]
struct StopsQuery {
    search: String,
}

async fn stops(Query(params): Query<StopsQuery>) -> impl IntoResponse {
    let mut connection = establish_connection();

    use crate::schema::stops::dsl::*;
    let results = stops
        .filter(name.ilike(format!("{}%", params.search)))
        .limit(25)
        .load::<Stop>(&mut connection)
        .expect("Error loading stops");

    Json(results)
}