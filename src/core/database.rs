use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use diesel::prelude::*;
use dotenv::dotenv;
use diesel::r2d2::{PooledConnection, Error };


type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

lazy_static! {
    static ref DB_CONNECTION_POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        // Pool::new(manager).expect("Failed to create db pool")
        Pool::builder().build(manager).expect("Failed to create database connection pool")
    };
}

pub fn init() {
    lazy_static::initialize(&DB_CONNECTION_POOL);
    let conn = connection().expect("Failed to create database connection pool");
    embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, Error> {
    DB_CONNECTION_POOL.get()
        .map_err(|e|  ::new(500, format!("Failed getting db connection: {}", e)))
}