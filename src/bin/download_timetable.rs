use std::fs::File;
use std::io::{Read, Write};
use buss2::models::Stop;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_xml_rs;

#[tokio::main]
async fn main() {
// Download and parse XML
//     let body = load_estimated_timetables("buss2").await;
    let mut file = File::open("akt.xml").unwrap();
    let mut body = String::new();
    file.read_to_string(&mut body).unwrap();

    let siri: Siri = serde_xml_rs::from_str(&body).unwrap();

    println!("{:?}", siri.service_delivery.estimated_time_table_delivery.estimated_journey_version_frame.estimated_vehicle_journey.len());

    // // Write body to file
    // let mut file = File::create("akt.xml").unwrap();
    // file.write_all(body.as_bytes()).unwrap();


    // let stops: Vec<Stop> = serde_xml_rs::from_str(&body).unwrap();
    // println!("{:?}", stops);
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