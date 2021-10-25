use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use crate::config;

embed_migrations!();

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static::lazy_static! {
    static ref DB_CONNECTION_POOL: Pool = {
        let manager = ConnectionManager::<PgConnection>::new(&*config::CONFIG.database_url);
        Pool::builder().build(manager).expect("Failed to create database connection pool")
    };
}

pub fn init_and_run_migration() {
    let conn = get_database_connection().expect("Failed to get database connection pool");
    // run_pending_migrations(&conn);
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).expect("Run Postgres Migration Fail");
}

pub fn get_database_connection() -> Result<DbConnection, r2d2::Error> {
    DB_CONNECTION_POOL.get()
}