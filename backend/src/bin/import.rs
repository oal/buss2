use csv::Reader;
use diesel::prelude::*;
use diesel::upsert::excluded;
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
    use buss2::schema::stops::dsl::*;

    let mut rdr = read_stops_file();
    let new_stops = rdr.records().flat_map(|result| {
        let record = result?;
        let id_str = record[0].to_string();
        if id_str.starts_with("NSR:StopPlace") {
            Ok(Stop {
                id: get_last_as_i32(&id_str),
                name: record[1].to_string(),
                lat: record[2].parse().unwrap(),
                lon: record[3].parse().unwrap(),
            })
        } else {
            Err(anyhow::Error::msg("Not a stop"))
        }
    }).collect::<Vec<_>>();
    println!("Found {} stops.", new_stops.len());

    let num_inserted = diesel::insert_into(stops)
        .values(&new_stops)
        .on_conflict(id)
        .do_update()
        .set((
            name.eq(excluded(name)),
            lat.eq(excluded(lat)),
            lon.eq(excluded(lon)),
        ))
        .execute(&mut connection).await
        .expect("Error saving stop.");
    println!("Inserted {} stops.", num_inserted);
}

async fn import_quays(pool: DbPool) {
    let mut connection = pool.get().await.unwrap();
    use buss2::schema::quays::dsl::*;

    let mut rdr = read_stops_file();
    let new_quays = rdr.records().flat_map(|result| {
        let record = result.unwrap();
        let id_str = record[0].to_string();
        if id_str.starts_with("NSR:Quay") {
            Ok(Quay {
                id: get_last_as_i32(&id_str),
                name: record[1].to_string(),
                lat: record[2].parse().unwrap(),
                lon: record[3].parse().unwrap(),
                stop_id: get_last_as_i32(&record[6]),
            })
        } else {
            Err(anyhow::Error::msg("Not a quay"))
        }
    }).collect::<Vec<_>>();
    println!("Found {} quays.", new_quays.len());

    let num_inserted = diesel::insert_into(quays)
        .values(&new_quays)
        .on_conflict(id)
        .do_update()
        .set((
            name.eq(excluded(name)),
            lat.eq(excluded(lat)),
            lon.eq(excluded(lon)),
            stop_id.eq(excluded(stop_id))
        ))
        .execute(&mut connection)
        .await
        .expect("Error saving quay.");
    println!("Inserted {} quays.", num_inserted);
}

async fn import_routes(pool: DbPool) {
    let mut connection = pool.get().await.unwrap();
    use buss2::schema::routes::dsl::*;

    let mut rdr = read_routes_file();
    let new_routes = rdr.records().flat_map(|result| {
        if let Ok(record) = &result {
            Some(Route {
                id: get_last_as_i32(&record[1]),
                short_name: record[2].to_string(),
                name: record[3].to_string(),
            })
        } else {
            None
        }
    }).collect::<Vec<_>>();

    println!("Found {} routes.", new_routes.len());

    let num_inserted = diesel::insert_into(routes)
        .values(&new_routes)
        .on_conflict(id)
        .do_update()
        .set((
            short_name.eq(excluded(short_name)),
            name.eq(excluded(name))
        ))
        .execute(&mut connection)
        .await
        .expect("Error saving route.");
    println!("Inserted {} routes.", num_inserted);
}