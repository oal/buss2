use std::ops::Sub;
use diesel::prelude::*;
use diesel_async::{RunQueryDsl};
use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use chrono::{Utc};
use diesel::query_dsl::methods::DistinctOnDsl;
use serde::Serialize;
use crate::db::{DbConnection, DbPool};

#[derive(Queryable, Serialize)]
struct QuayRoute {
    id: i32,
    short_name: String,
    name: String,
}

pub(crate) async fn show(
    State(pool): State<DbPool>,
    Path(_id): Path<i32>,
) -> impl IntoResponse {
    let mut connection = pool.get().await.unwrap();

    let result = get_quay_routes(&mut connection, _id).await;
    Json(result)
}

async fn get_quay_routes(connection: &mut DbConnection, _id: i32) -> Vec<QuayRoute> {
    let now = Utc::now();
    let a_week_ago = now.sub(chrono::Duration::weeks(1));

    // Get routes that have visited this quay in the last week.
    use crate::schema::routes::dsl::*;
    use crate::schema::estimated_calls;
    routes.select((id, short_name, name))
        .inner_join(crate::schema::journeys::table.inner_join(estimated_calls::table))
        .filter(estimated_calls::expected_arrival_time.lt(now))
        .filter(estimated_calls::expected_arrival_time.gt(a_week_ago))
        .filter(estimated_calls::quay_id.eq(_id))
        .distinct_on(id)
        .load(connection)
        .await
        .expect("Error loading routes")
}