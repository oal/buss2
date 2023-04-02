use axum::extract::{Path, Query};
use axum::Json;
use axum::response::IntoResponse;
use diesel::prelude::*;
use crate::db::establish_connection;
use crate::models::{Quay, Stop};

#[derive(serde::Deserialize)]
pub(crate) struct StopsQuery {
    search: String,
}

pub(crate) async fn list(Query(params): Query<StopsQuery>) -> impl IntoResponse {
    let mut connection = establish_connection();

    use crate::schema::stops::dsl::*;
    let results = stops
        .filter(name.ilike(format!("{}%", params.search)))
        .limit(25)
        .load::<Stop>(&mut connection)
        .expect("Error loading stops");

    Json(results)
}

pub(crate) async fn show(Path(_id): Path<i32>) -> impl IntoResponse {
    let mut connection = establish_connection();

    use crate::schema::quays::dsl::*;
    if let Ok(result) = quays.filter(stop_id.eq(_id)).load::<Quay>(&mut connection) {
        Json(Some(result))
    } else {
        Json(None)
    }
}
