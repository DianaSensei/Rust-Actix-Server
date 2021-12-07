use once_cell::sync::OnceCell;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::time::Duration;

use crate::config;

static KAFKA_CONNECTION: OnceCell<FutureProducer> = OnceCell::new();
static KAFKA_CONNECTION_INITIALIZED: OnceCell<tokio::sync::Mutex<bool>> = OnceCell::new();

pub async fn get_kafka_connection() -> Option<&'static FutureProducer> {
    if KAFKA_CONNECTION.get().is_some() {
        return KAFKA_CONNECTION.get();
    }

    let initializing_mutex =
        KAFKA_CONNECTION_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));
    let mut initialized = initializing_mutex.lock().await;

    if !*initialized {
        if let Ok(conn) = ClientConfig::new()
            .set(
                "bootstrap.servers",
                config::CONFIG.kafka_broker_url.as_str(),
            )
            .set(
                "message.timeout.ms",
                config::CONFIG.kafka_message_timeout.as_str(),
            )
            .create()
        {
            if KAFKA_CONNECTION.set(conn).is_ok() {
                info!("KAFKA CLIENT INITIATE: [SUCCESS]");
                *initialized = true;
            }
        }
    }

    drop(initialized);
    KAFKA_CONNECTION.get()
}

async fn send_message(producer: FutureProducer, topic: String, payload: String) {
    let produce_future = producer.send::<String, _, _>(
        FutureRecord::to(topic.as_str()).payload(payload.as_str()),
        Duration::from_secs(0),
    );

    info!(
        "Send Kafka Message to topic `{}`: body `{}`",
        topic, payload
    );

    match produce_future.await {
        Ok(delivery) => println!("Kafka produce result: {:?}", delivery),
        Err((e, _)) => println!("Kafka produce error: {:?}", e),
    }
}
