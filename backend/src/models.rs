use diesel::prelude::*;
use crate::schema::stops;
use crate::schema::quays;
use crate::schema::routes;
use crate::schema::journeys;
use crate::schema::estimated_calls;
use chrono::{DateTime, Utc};
use serde::Serialize;
use ts_rs::TS;

#[derive(Queryable, Insertable, Identifiable, Associations, Serialize, TS, Debug)]
#[diesel(table_name = quays)]
#[diesel(belongs_to(Stop))]
#[ts(export)]
pub struct Quay {
    pub id: i32,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub stop_id: i32,
}

#[derive(Queryable, Insertable, Identifiable, Selectable, Serialize, TS, Debug)]
#[diesel(table_name = stops)]
#[ts(export)]
pub struct Stop {
    pub id: i32,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = routes)]
pub struct Route {
    pub id: i32,
    pub short_name: String,
    pub name: String,
}

#[derive(diesel_derive_enum::DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::sql_types::DirectionEnum"]
pub enum Direction {
    Outbound,
    Inbound,
}

#[derive(Insertable, AsChangeset, Debug)]
#[diesel(table_name = journeys)]
pub struct NewJourney {
    pub route_id: i32,
    pub journey_ref: String,
    pub direction: Direction,
}

#[derive(Insertable, AsChangeset, Debug)]
#[diesel(table_name = estimated_calls)]
pub struct NewEstimatedCall {
    pub journey_id: i32,
    pub order_in_journey: i32,

    pub quay_id: i32,

    pub target_arrival_time: Option<DateTime<Utc>>,
    pub target_departure_time: Option<DateTime<Utc>>,
    pub expected_arrival_time: Option<DateTime<Utc>>,
    pub expected_departure_time: Option<DateTime<Utc>>,
}

#[derive(Identifiable, Queryable, Selectable, Associations, Serialize, Debug)]
#[diesel(belongs_to(Quay))]
#[diesel(table_name = estimated_calls)]
pub struct EstimatedCall {
    pub id: i32,
    pub quay_id: i32,

    pub target_arrival_time: Option<DateTime<Utc>>,
    pub target_departure_time: Option<DateTime<Utc>>,
    pub expected_arrival_time: Option<DateTime<Utc>>,
    pub expected_departure_time: Option<DateTime<Utc>>,
}
