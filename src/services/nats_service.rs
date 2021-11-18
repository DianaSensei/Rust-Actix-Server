use crate::model::nats::*;
use crate::services::client::get_nats_connection;

pub async fn start_registered_consumer() {
    create_users_topic().await;
}

async fn create_users_topic() {
    let topic = "TEST1.abc.dd";
    let queue = "queue1.a.1";
    let nats_conn = get_nats_connection()
        .await
        .expect("Get Nats Connection Fail");
    match nats_conn.queue_subscribe(topic, queue) {
        Err(e) => error!(
            "[NATS] Create queue subscriber for topic `{}` queue `{}` fail | {}",
            topic, queue, e
        ),
        Ok(sub) => {
            info!("Subscribe topic `{}` queue `{}` success", topic, queue);
            sub.with_handler(move |msg| {
                let nats_req = NatsRequest::from(msg.clone());
                info!(
                    "[IN] topic `{}` queue `{}`: message: {:?}",
                    topic, queue, nats_req
                );
                msg.respond("")
            });
        }
    }
}
