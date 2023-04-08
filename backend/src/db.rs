use std::env;
use diesel::{Connection, PgConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::{Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub type DbPool = Pool<diesel_async::AsyncPgConnection>;
pub type DbConnection = diesel_async::AsyncPgConnection;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    println!("Running migrations...");
    connection.run_pending_migrations(MIGRATIONS).unwrap();
    println!("Migrations done.");
}

pub async fn create_db_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);

    Pool::builder(config).build().unwrap()
}

