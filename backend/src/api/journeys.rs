use axum::extract::{Path, Query, State};
use axum::Json;
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Serialize, Deserialize};
use ts_rs::TS;
use crate::db::DbPool;
use crate::models::{Route};
use crate::schema::estimated_calls;
use crate::schema::quays;

#[derive(Deserialize)]
pub struct JourneyListParams {
    quay: i32,
    routes: Option<String>,
}

impl JourneyListParams {
    pub fn routes(&self) -> anyhow::Result<Vec<i32>> {
        if let Some(routes) = &self.routes {
            let ids = routes
                .split(',')
                .filter_map(|route| route.parse::<i32>().ok())
                .collect();
            Ok(ids)
        } else {
            Err(anyhow::anyhow!("No routes provided"))
        }
    }
}

#[derive(Queryable, Selectable, Serialize, TS)]
#[diesel(table_name = estimated_calls)]
#[ts(export)]
struct EstimatedCall {
    id: i32,
    target_arrival_time: Option<DateTime<Utc>>,
    expected_arrival_time: Option<DateTime<Utc>>,
    target_departure_time: Option<DateTime<Utc>>,
    expected_departure_time: Option<DateTime<Utc>>,
}

#[derive(Queryable, Serialize, TS)]
#[ts(export)]
struct Departure {
    id: i32,
    route: Route,
    estimated_call: EstimatedCall,
}

pub async fn list(
    State(pool): State<DbPool>,
    Query(params): Query<JourneyListParams>,
) -> impl IntoResponse {
    let mut connection = pool.get().await.unwrap();
    let now = Utc::now();

    use crate::schema::journeys;
    use crate::schema::routes;
    let mut departures_query = journeys::table
        .inner_join(routes::table)
        .inner_join(estimated_calls::table)
        .select((journeys::id, Route::as_select(), EstimatedCall::as_select()))
        .filter(estimated_calls::quay_id.eq(params.quay))
        .filter(estimated_calls::expected_arrival_time.ge(now))
        .order(estimated_calls::expected_arrival_time.asc())
        .into_boxed();

    if let Ok(routes) = params.routes() {
        departures_query = departures_query.filter(journeys::route_id.eq_any(routes));
    }

    let departures = departures_query
        .load::<Departure>(&mut connection)
        .await
        .unwrap();

    Json(departures)
}

#[derive(Queryable, Selectable, Serialize, TS)]
#[diesel(table_name = quays)]
#[ts(export)]
struct Quay {
    id: i32,
    name: String,
}

#[derive(Queryable, Serialize, TS)]
#[ts(export)]
struct EstimatedCallWithQuay {
    #[serde(flatten)]
    estimated_call: EstimatedCall,
    quay: Quay,
}

#[derive(Queryable, Serialize, TS)]
#[ts(export)]
struct Journey {
    id: i32,
    route: Route,
    estimated_calls: Vec<EstimatedCallWithQuay>,
}

pub async fn show(
    State(pool): State<DbPool>,
    Path(_id): Path<i32>,
) -> impl IntoResponse {
    let mut connection = pool.get().await.unwrap();

    let (_journey_id, route) = {
        use crate::schema::journeys::dsl::*;
        journeys.find(_id)
            .inner_join(crate::schema::routes::table)
            .select((id, Route::as_select()))
            .first::<(i32, Route)>(&mut connection)
            .await
            .unwrap()
    };

    let calls = {
        use crate::schema::estimated_calls::dsl::*;
        estimated_calls
            .filter(journey_id.eq(_id))
            .inner_join(quays::table)
            .select((EstimatedCall::as_select(), Quay::as_select()))
            .order(order_in_journey.asc())
            .load::<EstimatedCallWithQuay>(&mut connection)
            .await
            .unwrap()
    };

    Json(Journey {
        id: _journey_id,
        route,
        estimated_calls: calls,
    })
}

#[derive(Deserialize)]
pub struct FavoritesQuery {
    ids: String
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
                let route = parts.next();
                let quay = parts.next();
                if let (Some(route), Some(quay)) = (route, quay) {
                    let route = route.parse::<i32>().ok();
                    let quay = quay.parse::<i32>().ok();
                    if let (Some(route), Some(quay)) = (route, quay) {
                        Some(FavoritePair {route, quay})
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

pub async fn favorites(
    State(pool): State<DbPool>,
    Query(params): Query<FavoritesQuery>,
) -> impl IntoResponse {
    // let mut connection = pool.get().await.unwrap();

    let pairs = params.route_quay_pairs().unwrap();

    // use crate::schema::journeys;
    // use crate::schema::routes;
    // let departures = journeys::table
    //     .inner_join(routes::table)
    //     .inner_join(estimated_calls::table)
    //     .select((journeys::id, Route::as_select(), EstimatedCall::as_select()))
    //     .filter(estimated_calls::quay_id.eq(1))
    //     .filter(estimated_calls::expected_arrival_time.ge(Utc::now()))
    //     .order(estimated_calls::expected_arrival_time.asc())
    //     .load::<Departure>(&mut connection)
    //     .await
    //     .unwrap();

    Json(pairs)
}