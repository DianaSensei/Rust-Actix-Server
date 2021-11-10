pub mod client;
mod nats_service;
mod web_service;

pub use nats_service::start_registered_consumer;
pub use web_service::start_web_service;
