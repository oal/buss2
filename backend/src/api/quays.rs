use std::ops::Sub;
use diesel::prelude::*;
use diesel_async::{RunQueryDsl};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::{Utc};
use serde::Serialize;
use ts_rs::TS;
use crate::db::{DbConnection, DbPool};
use crate::models::Quay;

#[derive(Queryable, Serialize, TS)]
#[ts(export)]
pub struct QuayAugmented {
    #[serde(flatten)]
    quay: Quay,
    routes: Vec<QuayRoute>,
}

pub(crate) async fn show(
    State(pool): State<DbPool>,
    Path(_id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut connection = pool.get().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let quay = get_quay(&mut connection, _id).await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(QuayAugmented {
        quay,
        routes: get_quay_routes(&mut connection, _id).await,
    }))
}

async fn get_quay(connection: &mut DbConnection, _id: i32) -> Result<Quay, diesel::result::Error> {
    use crate::schema::quays::dsl::*;
    quays.find(_id).first::<Quay>(connection).await
}

#[derive(Queryable, Serialize, TS)]
#[ts(export)]
pub struct QuayRoute {
    id: i32,
    short_name: String,
    name: String,
}

async fn get_quay_routes(connection: &mut DbConnection, quay_id: i32) -> Vec<QuayRoute> {
    let now = Utc::now();
    let a_week_ago = now.sub(chrono::Duration::weeks(1));

    // Get routes that have visited this quay in the last week.
    use crate::schema::routes::dsl::*;
    use crate::schema::estimated_calls;
    routes.select((id, short_name, name))
        .inner_join(crate::schema::journeys::table.inner_join(estimated_calls::table))
        .filter(estimated_calls::expected_arrival_time.lt(now))
        .filter(estimated_calls::expected_arrival_time.gt(a_week_ago))
        .filter(estimated_calls::quay_id.eq(quay_id))
        .distinct_on(id)
        .load(connection)
        .await
        .expect("Error loading routes")
}