use std::net::SocketAddr;
use std::time::Duration;
use axum::{Router, ServiceExt};
use axum::routing::get;
use tokio::task;
use tokio::time::sleep;
use buss2::schema::quays;
use buss2::timetables::sync_timetables;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .nest("/api", buss2::api::api_router());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // sync_timetables_forever();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> &'static str {
    // let mut connection = buss2::db::establish_connection();
    // let quay_id = diesel::select(quays::id)
    //     .from(quays::table)
    //     .first::<i32>(&mut connection)
    //     .unwrap();

    // &format!("Hello, World! {quay_id}")
    "Hello, World!"
}

fn sync_timetables_forever() {
    task::spawn(async {
        let mut connection = buss2::db::establish_connection();
        loop {
            sync_timetables("1828b7c2-fcc8-47f4-b6cc-541e0015a8d5", &mut connection).await;
            println!("Synced timetables.");
            sleep(Duration::from_secs(60)).await;
        }
    });
}