use nats::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NatsRequest {
    pub id: String,
    pub from: String,
    pub data: Json,
    pub send_time: i64,
}

impl From<Message> for NatsRequest {
    fn from(msg: Message) -> Self {
        serde_json::from_slice(&msg.data).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NatsResponse {
    pub request: Option<NatsRequest>,
    pub id: String,
    pub from: String,
    pub data: Json,
    pub send_time: i64,
    pub status: i16,
    pub status_des: String,
}

impl From<Message> for NatsResponse {
    fn from(msg: Message) -> Self {
        serde_json::from_slice(&msg.data).unwrap()
    }
}
