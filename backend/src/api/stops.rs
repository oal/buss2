use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use diesel::prelude::*;
use diesel_async::{RunQueryDsl};
use serde::Serialize;
use ts_rs::TS;
use crate::db::{DbPool};
use crate::models::{EstimatedCall, Quay, Stop};

#[derive(serde::Deserialize)]
pub(crate) struct StopsQuery {
    search: String,
}

pub(crate) async fn list(
    Query(params): Query<StopsQuery>,
    State(pool): State<DbPool>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut connection = pool.get().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    use crate::schema::stops::dsl::*;
    let results = stops
        .filter(name.ilike(format!("{}%", params.search)))
        .limit(25)
        .load::<Stop>(&mut connection)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(results))
}

#[derive(Serialize, TS)]
#[ts(export)]
struct StopWithQuays {
    #[serde(flatten)]
    stop: Stop,
    quays: Vec<Quay>,
}

#[derive(Serialize)]
struct QuayWithCalls {
    #[serde(flatten)]
    quay: Quay,
    calls: Vec<EstimatedCall>,
}

pub(crate) async fn show(
    Path(_id): Path<i32>,
    State(pool): State<DbPool>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut connection = pool.get().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stop = {
        use crate::schema::stops::dsl::*;
        stops
            .filter(id.eq(_id))
            .first::<Stop>(&mut connection)
            .await
            .expect("Error loading stop")
    };

    let quays = {
        use crate::schema::quays::dsl::*;
        quays
            .filter(stop_id.eq(_id))
            .load::<Quay>(&mut connection)
            .await
            .expect("Error loading quays")
    };

    Ok(Json(StopWithQuays {
        stop,
        quays,
    }))
}
