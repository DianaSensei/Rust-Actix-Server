use crate::model::nats_message::*;
use crate::model::*;
use crate::lib::nats_broker::*;
use actix::prelude::*;
use chrono::Utc;
use std::collections::HashMap;
use serde_json::{Value as Json};
use crate::config;

struct NatsActor {
    // conn: NatsConnection,
    // topic: String
}
#[derive(Message)]
#[rtype(result = "usize")]
struct Value(usize, usize);

// impl Json for Value {

// }
impl NatsActor {
        pub async fn subscribe(topic: String) {
        let conn = NatsServer::connect(config::CONFIG.nats_url.to_owned())
        .await
        .expect("Connect Nats Fail");
        match NatsServer::queue_subscribe(conn, topic.to_owned(), "".to_string()).await {
                    Ok(sub) => {
            println!("Consumer: `{}` ready", topic.clone());
            sub.with_handler(move |msg| {
                // let nats_req = NatsRequest::from(msg.clone());
                // let res = futures::executor::block_on(hello());
                // let nats_res = match res {
                //     Ok(user) => {
                //         resp_nats(
                //         nats_req,
                //         "resp_create_user".to_owned(),
                //         serde_json::to_value(&user).unwrap(),
                //         true,
                //         0,
                //         "Ok".to_owned(),
                //     )
                // },
                //     Err(e) => resp_nats(
                //         nats_req,
                //         "create_user".to_owned(),
                //         json!({}),
                //         false,
                //         -1,
                //         e.to_string(),
                //     ),
                // };
                msg.respond("")
            });
        }
        Err(e) => {
            println!(
                "[NATS][FAIL] Create subscriber for topic:`{}` fail | {}",
                topic, e
            );
        }
    }
        // actix_rt::spawn(call);
        // NatsActor { conn, topic}
    }
}

impl Actor for NatsActor {
    type Context = Context<Self>;
}

pub async fn start_nats_server(nats_conn: NatsConnection) {
    create_users_topic("user.create".to_owned(), nats_conn).await;
}

async fn create_users_topic(topic: String, nats_conn: NatsConnection) {
    match NatsServer::queue_subscribe(nats_conn, topic.to_owned(), "".to_string()).await {
        Ok(sub) => {
            println!("Consumer: `{}` ready", topic.clone());
            sub.with_handler(move |msg| {
                // let nats_req = NatsRequest::from(msg.clone());
                // let res = futures::executor::block_on(hello());
                // let nats_res = match res {
                //     Ok(user) => {
                //         resp_nats(
                //         nats_req,
                //         "resp_create_user".to_owned(),
                //         serde_json::to_value(&user).unwrap(),
                //         true,
                //         0,
                //         "Ok".to_owned(),
                //     )
                // },
                //     Err(e) => resp_nats(
                //         nats_req,
                //         "create_user".to_owned(),
                //         json!({}),
                //         false,
                //         -1,
                //         e.to_string(),
                //     ),
                // };
                msg.respond("")
            });
        }
        Err(e) => {
            println!(
                "[NATS][FAIL] Create subscriber for topic:`{}` fail | {}",
                topic, e
            );
        }
    }
}
async fn get_users_topic(topic: String, nats_conn: NatsConnection) {
    match NatsServer::queue_subscribe(nats_conn, topic.to_owned(), "".to_string()).await {
        Ok(sub) => {
            sub.with_handler(move |msg| {
                // let nats_res = NatsRequest::from(msg.clone());
                // let res = futures::executor::block_on(hello());
                // let res_data = serde_json::to_string(&res.unwrap()).unwrap();
                msg.respond("")
            });
        }
        Err(e) => {
            println!(
                "[NATS][FAIL] Create subscriber for topic:`{}` fail | {}",
                topic, e
            );
        }
    }
}

impl From<NatsRequest> for Register {
    fn from(nas_req: NatsRequest) -> Self {
        let doc = nas_req.data;
        let email = doc["email"].as_str().unwrap_or("").to_owned();
        let password = doc["password"].as_str().unwrap_or("").to_owned();
        Self {
            email: if email == "" {
                None
            } else {
                Some(email.to_string())
            },
            password: if password == "" {
                None
            } else {
                Some(password.to_string())
            },
        }
    }
}
impl From<NatsRequest> for HashMap<String, String> {
    fn from(_nats_req: NatsRequest) -> Self {
        HashMap::new()
    }
}

fn resp_nats(
    nats_req: NatsRequest,
    resp_type: String,
    data: serde_json::Value,
    status: bool,
    status_code: i64,
    status_des: String,
) -> NatsResponse {
    let now = Utc::now().timestamp();
    NatsResponse {
        nats_request: nats_req,
        response_type: resp_type.to_owned(),
        response_id: resp_type.to_owned() + &now.to_string(),
        from: "User Service".to_owned(),
        data: data,
        status: status,
        send_time: now,
        status_code: status_code,
        status_des: status_des,
    }
}

async fn hello(){}