use csv::Reader;
use dotenvy::dotenv;
use diesel_async::{RunQueryDsl};
use buss2::db::{create_db_pool, DbPool};
use buss2::helpers::get_last_as_i32;
use buss2::models::{Quay, Route, Stop};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = create_db_pool().await;

    println!("Import");
    import_stops(pool.clone()).await;
    import_quays(pool.clone()).await;
    import_routes(pool).await;
}

fn read_stops_file() -> Reader<std::fs::File> {
    let dir = "/home/olav/Downloads/rb_akt-aggregated-gtfs(1)/stops.txt";
    Reader::from_path(dir).unwrap()
}

fn read_routes_file() -> Reader<std::fs::File> {
    let dir = "/home/olav/Downloads/rb_akt-aggregated-gtfs(1)/routes.txt";
    Reader::from_path(dir).unwrap()
}

async fn import_stops(pool: DbPool) {
    let mut connection = pool.get().await.unwrap();
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
                .execute(&mut connection).await
                .expect("Error saving stop.");
        }
    }
}

async fn import_quays(pool: DbPool) {
    let mut connection = pool.get().await.unwrap();
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
                .execute(&mut connection)
                .await
                .expect("Error saving quay.");
        }
    }
}

async fn import_routes(pool: DbPool) {
    let mut connection = pool.get().await.unwrap();
    let mut rdr = read_routes_file();
    for result in rdr.records() {
        let record = result.unwrap();
        use buss2::schema::routes::dsl::*;
        let route = Route {
            id: get_last_as_i32(&record[1]),
            short_name: record[2].to_string(),
            name: record[3].to_string(),
        };

        println!("{:?}", route);

        diesel::insert_into(routes)
            .values(&route)
            .on_conflict_do_nothing()
            .execute(&mut connection)
            .await
            .expect("Error saving route.");
    }
}