use std::time::Instant;
use chrono::{DateTime, Utc};
use diesel_async::{RunQueryDsl};
use anyhow::Result;
use diesel::prelude::*;
use diesel::upsert::excluded;
use serde::{Deserialize, Serialize};
use tokio::task;
use crate::db::{DbPool};
use crate::helpers::get_last_as_i32;
use crate::models::{Direction, NewEstimatedCall, NewJourney};

pub async fn sync_timetables(requestor_id: &str, pool: DbPool) {
    println!("Syncing timetables... (requestor_id: {})", requestor_id);

    let now = Instant::now();
    let body = load_estimated_timetables(requestor_id).await;
    // let body = load_estimated_timetables_from_file(requestor_id).await;
    println!("Loaded in {} ms.", now.elapsed().as_millis());

    let now = Instant::now();
    let siri: Siri = serde_xml_rs::from_str(&body).unwrap();
    println!("Deserialized in {} ms.", now.elapsed().as_millis());

    let now = Instant::now();
    insert_journeys(siri, pool).await;
    println!("Inserted in {} ms.", now.elapsed().as_millis());
}

async fn load_estimated_timetables(requestor_id: &str) -> String {
    let url = "https://api.entur.io/realtime/v1/rest/et?datasetId=AKT&requestorId=".to_string() + requestor_id;
    let response = reqwest::get(url).await.unwrap();
    response.text().await.unwrap()
}

async fn load_estimated_timetables_from_file(_requestor_id: &str) -> String {
    std::fs::read_to_string("akt.xml").unwrap()
}

async fn insert_journeys(siri: Siri, pool: DbPool) {
    let raw_journeys = siri
        .service_delivery.estimated_time_table_delivery
        .estimated_journey_version_frame
        .estimated_vehicle_journey
        .unwrap_or(vec![]);

    let tasks = raw_journeys.into_iter().map(|journey| {
        let pool = pool.clone();
        task::spawn(async move {
            if let Ok(journey_id) = insert_journey(&journey, pool.clone()).await {
                insert_estimated_calls(journey_id, &journey.estimated_calls.estimated_call, pool.clone()).await;
            }
        })
    });

    futures::future::join_all(tasks).await;
}

async fn insert_journey(journey: &EstimatedVehicleJourney, pool: DbPool) -> Result<i32> {
    use crate::schema::journeys::dsl::*;
    let journey_ref_str = &journey.dated_vehicle_journey_ref;
    let journey = NewJourney {
        route_id: get_last_as_i32(&journey.line_ref),
        journey_ref: journey_ref_str.to_string(),
        direction: if journey.direction_ref == "Outbound" {
            Direction::Outbound
        } else {
            Direction::Inbound
        },
    };

    let mut connection = pool.get().await.unwrap();
    let result: (i32, i32, String, Direction) = diesel::insert_into(journeys)
        .values(&journey)
        .on_conflict(journey_ref)
        .do_update()
        .set(&journey)
        .get_result(&mut connection)
        .await
        .expect("Error saving journey.");

    Ok(result.0)
}


async fn insert_estimated_calls(internal_id: i32, calls: &[EstimatedCall], pool: DbPool) {
    let mut connection = pool.get().await.unwrap();
    use crate::schema::estimated_calls::dsl::*;
    let calls = calls.iter().map(|call| {
        NewEstimatedCall {
            journey_id: internal_id,
            order_in_journey: call.order.parse().unwrap(),

            quay_id: get_last_as_i32(&call.stop_point_ref),

            target_arrival_time: parse_time(&call.aimed_arrival_time),
            target_departure_time: parse_time(&call.aimed_arrival_time),

            expected_arrival_time: parse_time(&call.expected_arrival_time),
            expected_departure_time: parse_time(&call.expected_departure_time),
        }
    }).collect::<Vec<_>>();

    diesel::insert_into(estimated_calls)
        .values(calls)
        .on_conflict((journey_id, order_in_journey))
        .do_update()
        .set((
            target_arrival_time.eq(excluded(target_arrival_time)),
            target_departure_time.eq(excluded(target_departure_time)),
            expected_arrival_time.eq(excluded(expected_arrival_time)),
            expected_departure_time.eq(excluded(expected_departure_time))
        ))
        .execute(&mut connection)
        .await
        .expect("Error saving stop.");
}

fn parse_time(time: &Option<String>) -> Option<DateTime<Utc>> {
    match time {
        Some(time) => {
            if let Ok(parsed_time) = DateTime::parse_from_rfc3339(time) {
                Some(parsed_time.with_timezone(&Utc))
            } else {
                None
            }
        }
        _ => None,
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Siri {
    #[serde(rename = "ServiceDelivery")]
    service_delivery: ServiceDelivery,
}

#[derive(Debug, Deserialize, Serialize)]
struct ServiceDelivery {
    #[serde(rename = "EstimatedTimetableDelivery")]
    estimated_time_table_delivery: EstimatedTimeTableDelivery,
}

#[derive(Debug, Deserialize, Serialize)]
struct EstimatedTimeTableDelivery {
    #[serde(rename = "EstimatedJourneyVersionFrame")]
    estimated_journey_version_frame: EstimatedJourneyVersionFrame,
}

#[derive(Debug, Deserialize, Serialize)]
struct EstimatedJourneyVersionFrame {
    #[serde(rename = "EstimatedVehicleJourney")]
    estimated_vehicle_journey: Option<Vec<EstimatedVehicleJourney>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct EstimatedVehicleJourney {
    line_ref: String,
    direction_ref: String,
    dated_vehicle_journey_ref: String,
    estimated_calls: EstimatedCalls,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct EstimatedCalls {
    estimated_call: Vec<EstimatedCall>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct EstimatedCall {
    stop_point_ref: String,
    order: String,
    aimed_arrival_time: Option<String>,
    expected_arrival_time: Option<String>,
    aimed_departure_time: Option<String>,
    expected_departure_time: Option<String>,
}

