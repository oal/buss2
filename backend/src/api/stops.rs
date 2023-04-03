use axum::extract::{Path, Query, State};
use axum::Json;
use axum::response::IntoResponse;
use diesel::prelude::*;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{RunQueryDsl};
use crate::db::{DbPool};
use crate::models::{Quay, Stop};

#[derive(serde::Deserialize)]
pub(crate) struct StopsQuery {
    search: String,
}

pub(crate) async fn list(
    Query(params): Query<StopsQuery>,
    State(pool): State<DbPool>,
) -> impl IntoResponse {
    let mut connection = pool.get().await.unwrap();

    use crate::schema::stops::dsl::*;
    let results = stops
        .filter(name.ilike(format!("{}%", params.search)))
        .limit(25)
        .load::<Stop>(&mut connection)
        .await
        .expect("Error loading stops");

    Json(results)
}

pub(crate) async fn show(
    Path(_id): Path<i32>,
    State(pool): State<DbPool>,
) -> impl IntoResponse {
    let mut connection = pool.get().await.unwrap();

    use crate::schema::quays::dsl::*;
    if let Ok(result) = quays.filter(stop_id.eq(_id)).load::<Quay>(&mut connection).await {
        Json(Some(result))
    } else {
        Json(None)
    }
}
