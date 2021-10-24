pub mod client;
pub mod nats_server;
mod nats_consumer_service;
mod web_service;

pub use nats_consumer_service::start_registered_consumer;
pub use web_service::start_web_service;