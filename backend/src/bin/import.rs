use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel_async::pooled_connection::deadpool::Pool;
use buss2::db::{create_db_pool};
use buss2::helpers::get_last_as_i32;
use buss2::models::{Quay, Route, Stop};

#[tokio::main]
async fn main() {
    let mut connection = establish_connection();
    let pool = create_db_pool().await;

    println!("Import");
    import_stops(&pool);
    import_quays(&pool);
    import_routes(&pool);
}

fn read_stops_file() -> csv::Reader<std::fs::File> {
    let dir = "/home/olav/Downloads/rb_akt-aggregated-gtfs(1)/stops.txt";
    return csv::Reader::from_path(dir).unwrap();
}

fn read_routes_file() -> csv::Reader<std::fs::File> {
    let dir = "/home/olav/Downloads/rb_akt-aggregated-gtfs(1)/routes.txt";
    return csv::Reader::from_path(dir).unwrap();
}

fn import_stops(connection: &Pool) {

    let mut rdr = read_stops_file();
    for result in rdr.records() {
        let record = result.unwrap();
        let id_str = record[0].to_string();
        if id_str.starts_with("NSR:StopPlace") {
            use buss2::schema::stops::dsl::*;
            let stop = Stop {
                id: get_last_as_i32(&id_str),
                name: record[1].to_string(),
                lat: record[2].parse().unwrap(),
                lon: record[3].parse().unwrap(),
            };

            println!("{:?}", stop);

            diesel::insert_into(stops)
                .values(&stop)
                .on_conflict_do_nothing()
                .execute(connection)
                .expect("Error saving stop.");
        }
    }
}

fn import_quays(connection: &Pool<_>) {
    let mut rdr = read_stops_file();
    for result in rdr.records() {
        let record = result.unwrap();
        let id_str = record[0].to_string();
        if id_str.starts_with("NSR:Quay") {
            use buss2::schema::quays::dsl::*;
            let quay = Quay {
                id: get_last_as_i32(&id_str),
                name: record[1].to_string(),
                lat: record[2].parse().unwrap(),
                lon: record[3].parse().unwrap(),
                stop_id: get_last_as_i32(&record[6]),
            };

            println!("{:?}", quay);

            diesel::insert_into(quays)
                .values(&quay)
                .on_conflict_do_nothing()
                .execute(connection)
                .expect("Error saving quay.");
        }
    }
}

fn import_routes(connection: &Pool<_>) {
    let mut rdr = read_routes_file();
    for result in rdr.records() {
        let record = result.unwrap();
        use buss2::schema::routes::dsl::*;
        let route = Route {
            id: get_last_as_i32(&record[1].to_string()),
            short_name: record[2].to_string(),
            name: record[3].to_string(),
        };

        println!("{:?}", route);

        diesel::insert_into(routes)
            .values(&route)
            .on_conflict_do_nothing()
            .execute(connection)
            .expect("Error saving route.");
    }
}