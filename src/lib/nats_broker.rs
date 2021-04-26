use nats::*;

pub type NatsConnection = Connection;

#[derive(Clone)]
pub struct NatsServer {
    conn: NatsConnection
}

impl NatsServer {
    pub fn connect(url: String) -> std::io::Result<Self> {
        let conn = connect(url.as_str()).expect(&*format!("Connect to {} fail", url));
        Ok(NatsServer { conn})
    }
    pub async fn connect_with_user_pass(user_name: String, pass_word: String, url: String) -> std::io::Result<Self> {
        let conn = nats::Options::with_user_pass(user_name.as_str(), pass_word.as_str())
            .with_name("Rust NATS Client")
            .connect(url.as_str()).expect(&*format!("Connect to {} fail", url));
        Ok(NatsServer { conn})
    }
    pub async fn subscribe(&mut self, topic_name: String) -> std::io::Result<Subscription> {
        self.conn.subscribe(topic_name.as_str())
    }
    pub async fn queue_subscribe(&mut self, topic_name: String, queue: String) -> std::io::Result<Subscription> {
        self.conn.queue_subscribe(topic_name.as_str(), queue.as_str())
    }
}
