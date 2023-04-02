use std::fs::File;
use std::io::{Read, Write};
use diesel::prelude::*;
use buss2::models::{NewEstimatedCall, NewJourney, Stop};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_xml_rs;
use buss2::db::establish_connection;
use buss2::helpers::get_last_as_i32;
use chrono::{DateTime, Utc};

#[tokio::main]
async fn main() {
// Download and parse XML
//     let body = load_estimated_timetables("buss2").await;
    let mut file = File::open("akt.xml").unwrap();
    let mut body = String::new();
    file.read_to_string(&mut body).unwrap();

    let siri: Siri = serde_xml_rs::from_str(&body).unwrap();


    let mut connection = establish_connection();
    insert_journeys(siri, &mut connection);

    // // Write body to file
    // let mut file = File::create("akt.xml").unwrap();
    // file.write_all(body.as_bytes()).unwrap();


    // let stops: Vec<Stop> = serde_xml_rs::from_str(&body).unwrap();
    // println!("{:?}", stops);
}

fn insert_journeys(siri: Siri, mut connection: &mut PgConnection) {
    for journey in siri.service_delivery.estimated_time_table_delivery.estimated_journey_version_frame.estimated_vehicle_journey {
        println!("{:?}", journey);

        let journey_id = insert_journey(&journey, &mut connection);
        insert_estimated_calls(journey_id, &journey.estimated_calls.estimated_call, &mut connection);
    }
}

fn insert_journey(journey: &EstimatedVehicleJourney, mut connection: &mut PgConnection) -> i32 {
    use buss2::schema::journeys::dsl::*;
    let journey_ref_str = &journey.dated_vehicle_journey_ref;
    let journey = NewJourney {
        route_id: get_last_as_i32(&journey.line_ref),
        journey_ref: journey_ref_str.to_string(),
    };

    println!("{:?}", journey);

    let result: (i32, Option<i32>, String) = diesel::insert_into(journeys)
        .values(&journey)
        .on_conflict(journey_ref)
        .do_update()
        .set(&journey)
        .get_result(connection)
        .expect("Error saving stop.");

    return result.0;
}

fn parse_time(time: &Option<String>) -> Option<DateTime<Utc>> {
    match time {
        Some(time) => Some(DateTime::parse_from_rfc3339(time).unwrap().with_timezone(&Utc)),
        _ => None,
    }
}

fn insert_estimated_calls(internal_id: i32, calls: &Vec<EstimatedCall>, mut connection: &mut PgConnection) {
    use buss2::schema::estimated_calls::dsl::*;
    for call in calls {
        let estimated_call = NewEstimatedCall {
            journey_id: internal_id,
            order_in_journey: call.order.parse().unwrap(),

            quay_id: get_last_as_i32(&call.stop_point_ref),

            target_arrival_time: parse_time(&call.aimed_arrival_time),
            target_departure_time: parse_time(&call.aimed_arrival_time),

            expected_arrival_time: parse_time(&call.expected_arrival_time),
            expected_departure_time: parse_time(&call.expected_departure_time),
        };

        println!("{:?}", estimated_call);

        diesel::insert_into(estimated_calls)
            .values(&estimated_call)
            .on_conflict((journey_id, order_in_journey))
            .do_update()
            .set(&estimated_call)
            .execute(connection)
            .expect("Error saving stop.");
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
    estimated_vehicle_journey: Vec<EstimatedVehicleJourney>,
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

async fn load_estimated_timetables(requestor_id: &str) -> String {
    let url = "https://api.entur.io/realtime/v1/rest/et?datasetId=AKT&requestorId=".to_string() + requestor_id;
    let mut response = reqwest::get(url).await.unwrap();
    let body = response.text().await.unwrap().to_string();
    body
}