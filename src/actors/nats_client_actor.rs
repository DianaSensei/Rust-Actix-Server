use actix::prelude::*;
use nats::*;

pub struct NatsClientActor {
    server: Option<NatsClient>,
    url: String,
    topic: String
}

impl NatsClientActor {
    pub fn new(topic: String) -> Self {
        NatsClientActor {
            server: None,
            url: "".to_string(),
            topic
        }
    }
}

impl Actor for NatsClientActor {
    type Context = SyncContext<Self>;
    fn started(&mut self, _: &mut SyncContext<Self>) {
        let topic = &self.topic;
        match self.server {
            None => {
                self.server = Some(NatsClient::connect(topic).unwrap());
            }
            _ => {}
        }

        info!("Nats Listener Actor Started Up");
    }
    fn stopped(&mut self, _: &mut SyncContext<Self>) {
        info!("Nats Listener Actor Shutdown");
        System::current().stop();
    }
}

#[derive(Clone)]
pub struct NatsClient {
    conn: Connection
}

impl NatsClient {
    pub fn connect(url: &String) -> std::io::Result<Self> {
        let conn = connect(url.as_str()).expect(&*format!("Connect to Nats server: \"{}\" fail", url));
        Ok(NatsClient { conn })
    }

    pub async fn connect_with_user_pass(user_name: String, pass_word: String, url: &String) -> std::io::Result<Self> {
        let conn = nats::Options::with_user_pass(user_name.as_str(), pass_word.as_str())
            .connect(url.as_str())
            .expect(&*format!("Connect to Nats server: \"{}\" fail", url));

        Ok(NatsClient { conn })
    }

    pub async fn subscribe(&mut self, topic_name: String) -> std::io::Result<Subscription> {
        self.conn.subscribe(topic_name.as_str())
    }

    pub async fn queue_subscribe(&mut self, topic_name: String, queue: String) -> std::io::Result<Subscription> {
        self.conn.queue_subscribe(topic_name.as_str(), queue.as_str())
    }
}