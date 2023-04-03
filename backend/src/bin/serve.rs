use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use axum::{Router, ServiceExt};
use axum::handler::Handler;
use axum::routing::get;
use dotenvy::dotenv;
use tokio::task;
use tokio::time::sleep;
use buss2::db::{create_db_pool, DbPool};
use buss2::schema::quays;
use buss2::timetables::sync_timetables;


#[tokio::main]
async fn main() {
    dotenv().ok();

    // let app_state = AppState {
    // };

    let pool = create_db_pool().await;

    let app = Router::new()
        .route("/", get(index))
        .nest("/api", buss2::api::api_router(pool.clone()).await);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    sync_timetables_forever(pool);

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

fn sync_timetables_forever(pool: DbPool) {
    task::spawn(async move {
        loop {
            let now = std::time::Instant::now();
            sync_timetables("1828b7c2-fcc8-47f4-b6cc-541e0015a8d4", pool.clone()).await;
            println!("Synced timetables in {} ms.", now.elapsed().as_millis());
            sleep(Duration::from_secs(60)).await;
        }
    });
}