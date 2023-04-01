use diesel::prelude::*;
use crate::schema::stops;
use crate::schema::quays;

#[derive(Queryable, Insertable)]
#[diesel(table_name = quays)]
#[derive(Debug)]
pub struct Quay {
    pub id: i32,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub stop_id: i32,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = stops)]
#[derive(Debug)]
pub struct Stop {
    pub id: i32,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}