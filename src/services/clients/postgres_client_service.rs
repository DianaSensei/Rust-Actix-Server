use crate::settings;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::EmbeddedMigrations;
use once_cell::sync::Lazy;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

static DB_CONNECTION_POOL: Lazy<Pool> = Lazy::new(|| {
    let manager = ConnectionManager::<PgConnection>::new(&*settings::SETTINGS.datasource.database.url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool");
    info!("POSTGRES CLIENT INITIATE: [SUCCESS]");
    pool
});

// pub fn init_and_run_migration() {
//     let conn = get_database_connection();
//     // run_pending_migrations(&conn);
//     //
//     // (MIGRATIONS).unwrap();
//     embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
//         .expect("Run Postgres Migration Fail");
// }

pub fn get_database_connection() -> DbConnection {
    DB_CONNECTION_POOL
        .get()
        .expect("Get database connection error")
}
