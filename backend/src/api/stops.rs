use axum::extract::{Path, Query, State};
use axum::Json;
use axum::response::IntoResponse;
use diesel::prelude::*;
use diesel_async::{RunQueryDsl};
use serde::Serialize;
use crate::db::{DbPool};
use crate::models::{EstimatedCall, Quay, Stop};

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

#[derive(Serialize)]
struct StopWithQuays {
    #[serde(flatten)]
    stop: Stop,
    quays: Vec<Quay>,
    // quays: Vec<QuayWithCalls>,
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
) -> impl IntoResponse {
    let mut connection = pool.get().await.unwrap();


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

    // let next_calls = {
    //     use crate::schema::estimated_calls::dsl::*;
    //     let now = Utc::now();
    //     EstimatedCall::belonging_to(&quays)
    //         .select(EstimatedCall::as_select())
    //         .filter(expected_arrival_time.gt::<Option<chrono::DateTime<Utc>>>(Some(now)))
    //         .order(expected_arrival_time)
    //         .limit(5)
    //         .load::<EstimatedCall>(&mut connection)
    //         .await
    //         .expect("Error loading estimated calls")
    // };
    //
    // let quays_and_calls = next_calls
    //     .grouped_by(&quays)
    //     .into_iter()
    //     .zip(quays)
    //     .map(|(calls, quay)| QuayWithCalls {
    //         quay,
    //         calls,
    //     })
    //     .collect::<Vec<_>>();


    Json(StopWithQuays {
        stop,
        // quays: quays_and_calls
        quays
    })
}
