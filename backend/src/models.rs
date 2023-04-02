use diesel::prelude::*;
use crate::schema::stops;
use crate::schema::quays;
use crate::schema::routes;
use crate::schema::journeys;
use crate::schema::estimated_calls;
use chrono::{DateTime, Utc};
use diesel::{AsExpression, FromSqlRow};
use serde::Serialize;

#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = quays)]
#[derive(Debug)]
pub struct Quay {
    pub id: i32,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub stop_id: i32,
}

#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = stops)]
#[derive(Debug)]
pub struct Stop {
    pub id: i32,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = routes)]
#[derive(Debug)]
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

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = journeys)]
#[derive(Debug)]
pub struct NewJourney {
    pub route_id: i32,
    pub journey_ref: String,
    pub direction: Direction,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = estimated_calls)]
#[derive(Debug)]
pub struct NewEstimatedCall {
    pub journey_id: i32,
    pub order_in_journey: i32,

    pub quay_id: i32,

    pub target_arrival_time: Option<DateTime<Utc>>,
    pub target_departure_time: Option<DateTime<Utc>>,
    pub expected_arrival_time: Option<DateTime<Utc>>,
    pub expected_departure_time: Option<DateTime<Utc>>,
}