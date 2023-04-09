use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use axum::Router;
use axum::routing::get;
use dotenvy::dotenv;
use tokio::task;
use tokio::time::sleep;
use buss2::db::{create_db_pool, DbPool, run_migrations};
use buss2::timetables::sync_timetables;


#[tokio::main]
async fn main() {
    dotenv().ok();
    run_migrations();

    let pool = create_db_pool().await;

    let app = Router::new()
        .route("/", get(index))
        .nest("/api", buss2::api::api_router(pool.clone()).await);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3055));

    let requestor_id = env::var("ENTUR_SESSION").expect("ENTUR_SESSION must be set");
    sync_timetables_forever(pool, requestor_id);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn index() -> &'static str {
    ""
}

fn sync_timetables_forever(pool: DbPool, requestor_id: String) {
    task::spawn(async move {
        loop {
            let now = std::time::Instant::now();
            sync_timetables(&requestor_id, pool.clone()).await;
            println!("Synced timetables in {} ms.", now.elapsed().as_millis());
            sleep(Duration::from_secs(20)).await;
        }
    });
}