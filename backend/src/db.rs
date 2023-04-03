use std::env;
use diesel::{Connection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::{Object, Pool};
use dotenvy::dotenv;

// pub fn create_db_pool() -> Pool {
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let manager = Manager::new(&database_url, Runtime::Tokio1);
//     let pool = Pool::builder(manager)
//         .max_size(8)
//         .build()
//         .unwrap();
//     pool
// }
pub type DbPool = Pool<diesel_async::AsyncPgConnection>;
pub type DbConnection = diesel_async::AsyncPgConnection;

pub async fn create_db_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config).build().unwrap();
    pool
}

// #[derive(Clone)]
// pub struct AppState {
//     pub pool: DbPool,
// }
