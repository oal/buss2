use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use buss2::db::establish_connection;
use buss2::helpers::get_last_as_i32;
use buss2::models::{Quay, Route, Stop};

fn main() {
    let mut connection = establish_connection();

    println!("Import");
    import_stops(&mut connection);
    import_quays(&mut connection);
    import_routes(&mut connection);
}

fn read_stops_file() -> csv::Reader<std::fs::File> {
    let dir = "/home/olav/Downloads/rb_akt-aggregated-gtfs(1)/stops.txt";
    return csv::Reader::from_path(dir).unwrap();
}

fn read_routes_file() -> csv::Reader<std::fs::File> {
    let dir = "/home/olav/Downloads/rb_akt-aggregated-gtfs(1)/routes.txt";
    return csv::Reader::from_path(dir).unwrap();
}

fn import_stops(connection: &mut PgConnection) {
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

fn import_quays(connection: &mut PgConnection) {
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

fn import_routes(connection: &mut PgConnection) {
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