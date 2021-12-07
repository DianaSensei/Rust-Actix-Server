mod kafka_client_service;
mod mail_client_service;
mod nats_client_service;
pub mod postgres_client_service;
mod redis_client_service;
mod rest_client_service;

pub use kafka_client_service::get_kafka_connection;
pub use mail_client_service::get_smtp_connection;
pub use mail_client_service::send_email;
pub use nats_client_service::get_nats_connection;
pub use postgres_client_service::get_database_connection;
pub use redis_client_service::get_redis_connection;
pub use rest_client_service::get_rest_client;
