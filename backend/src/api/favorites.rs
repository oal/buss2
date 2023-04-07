use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Serialize, Deserialize};
use tokio::task;
use ts_rs::TS;
use crate::db::DbPool;
use crate::models::{EstimatedCall, Route, SimpleQuay};
use crate::schema::{estimated_calls, quays};

#[derive(Deserialize)]
pub struct FavoritesQuery {
    ids: String,
}

#[derive(Serialize)]
pub struct FavoritePair {
    route: i32,
    quay: i32,
}

impl FavoritesQuery {
    pub fn route_quay_pairs(&self) -> anyhow::Result<Vec<FavoritePair>> {
        let ids = self.ids
            .split(',')
            .filter_map(|route| {
                let mut parts = route.split('|');
                let quay = parts.next();
                let route = parts.next();
                if let (Some(route), Some(quay)) = (route, quay) {
                    let route = route.parse::<i32>().ok();
                    let quay = quay.parse::<i32>().ok();
                    if let (Some(route), Some(quay)) = (route, quay) {
                        Some(FavoritePair { route, quay })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        Ok(ids)
    }
}

#[derive(Serialize, TS)]
#[ts(export)]
struct FavoriteResult {
    journey_id: i32,
    route: Route,
    quay: SimpleQuay,
    estimated_call: EstimatedCall,
}

pub async fn list(
    State(pool): State<DbPool>,
    Query(params): Query<FavoritesQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    use crate::schema::journeys;
    use crate::schema::routes;

    let pairs = params
        .route_quay_pairs()
        .unwrap_or(vec![]);

    let now = Utc::now();
    let in_an_hour = now + Duration::hours(1);

    let tasks = pairs.into_iter().map(|pair| {
        let pool = pool.clone();
        task::spawn(async move {
            if let Ok(mut connection) = pool.get().await {
                estimated_calls::table
                    .inner_join(journeys::table.inner_join(routes::table))
                    .inner_join(quays::table)
                    .filter(journeys::route_id.eq(pair.route).and(estimated_calls::quay_id.eq(pair.quay)))
                    .filter(estimated_calls::expected_arrival_time.ge(now))
                    .filter(estimated_calls::expected_arrival_time.le(in_an_hour))
                    .select((journeys::id, EstimatedCall::as_select(), SimpleQuay::as_select(), Route::as_select()))
                    .order(estimated_calls::expected_arrival_time.asc())
                    .load::<(i32, EstimatedCall, SimpleQuay, Route)>(&mut connection)
                    .await
            } else {
                Err(diesel::result::Error::NotFound)
            }
        })
    });

    let results: Vec<_> = futures::future::join_all(tasks)
        .await
        .into_iter()
        .flatten()
        .filter_map(|result| result.ok())
        .flatten()
        .collect();

    let mut results = results
        .into_iter()
        .map(|tuple| FavoriteResult {
            journey_id: tuple.0,
            route: tuple.3,
            quay: tuple.2,
            estimated_call: tuple.1,
        })
        .collect::<Vec<_>>();

    results.sort_by(|a, b| {
        a.estimated_call.expected_arrival_time.cmp(&b.estimated_call.expected_arrival_time)
    });

    Ok(Json(results))
}
