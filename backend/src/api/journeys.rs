use axum::extract::{Query, State};
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
    target_arrival_time: Option<DateTime<Utc>>,
    expected_arrival_time: Option<DateTime<Utc>>,
    target_departure_time: Option<DateTime<Utc>>,
    expected_departure_time: Option<DateTime<Utc>>,
}

#[derive(Queryable, Serialize, TS)]
#[ts(export)]
struct Journey {
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
    let mut journies_query = journeys::table
        .inner_join(routes::table)
        .inner_join(estimated_calls::table)
        .select((journeys::id, Route::as_select(), EstimatedCall::as_select()))
        .filter(estimated_calls::quay_id.eq(params.quay))
        .filter(estimated_calls::expected_arrival_time.ge(now))
        .into_boxed();

    if let Ok(routes) = params.routes() {
        journies_query = journies_query.filter(journeys::route_id.eq_any(routes));
    }

    let journies = journies_query
        .load::<Journey>(&mut connection)
        .await
        .unwrap();

    Json(journies)
}