use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use buss2::models::{Quay, Stop};


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let mut connection = establish_connection();

    println!("Import");
    let dir = "/home/olav/Downloads/rb_akt-aggregated-gtfs(1)/stops.txt";

    let mut rdr = csv::Reader::from_path(dir).unwrap();
    for result in rdr.records() {
        let record = result.unwrap();

        let id_str = record[0].to_string();
        if id_str.starts_with("NSR:StopPlace") {
            use buss2::schema::quays::dsl::*;
            let quay = Quay {
                id: id_str.split(':').last().unwrap().parse().unwrap(),
                name: record[1].to_string(),
                lat: record[2].parse().unwrap(),
                lon: record[3].parse().unwrap(),
            };

            println!("{:?}", quay);

            diesel::insert_into(quays)
                .values(&quay)
                .on_conflict_do_nothing()
                .execute(&mut connection)
                .expect("Error saving quay.");
        } else if id_str.starts_with("NSR:StopPlace") {
            use buss2::schema::stops::dsl::*;
            let stop = Stop {
                id: id_str.split(':').last().unwrap().parse().unwrap(),
                name: record[1].to_string(),
                lat: record[2].parse().unwrap(),
                lon: record[3].parse().unwrap(),
            };

            println!("{:?}", stop);

            diesel::insert_into(stops)
                .values(&stop)
                .on_conflict_do_nothing()
                .execute(&mut connection)
                .expect("Error saving stop.");
        }
    }
}