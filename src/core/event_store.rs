use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use diesel::prelude::*;
use diesel_migrations::find_migrations_directory;
use dotenv::dotenv;
use diesel::r2d2::Error;
// use die
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// embed_migrations!();

lazy_static! {
    static ref DB_CONNECTION_POOL: Pool = {
        let db_url = std::env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
        Pool::builder().build(manager).expect("Failed to create persistent connection pool")
    };
}

pub fn init() {
    lazy_static::initialize(&DB_CONNECTION_POOL);
    let conn = connection().expect("Failed to create persistent connection pool");
    let migrations = find_migrations_directory().unwrap();
    conn.run_pending_migrations(migrations).unwrap();
    conn.begin_test_transaction().unwrap();
    // embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, Error> {
    DB_CONNECTION_POOL.get()
        .map_err(|e|  Error::new(500, format!("Failed getting db connection: {}", e)))
}