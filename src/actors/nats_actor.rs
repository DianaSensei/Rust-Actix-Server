use actix::prelude::*;
use std::{thread, time};
use crate::lib::nats_broker::*;
use std::borrow::Borrow;

#[derive(Default)]
pub struct NatsTask;
impl Message for NatsTask {
    type Result = ();
}

pub struct NatsActor {
    server: Option<NatsServer>,
    topic: String
}

impl NatsActor {
    fn new(topic: String) -> Self {
        NatsActor {
            server: None,
            topic
        }
    }
}

impl Actor for NatsActor {
    type Context = SyncContext<Self>;
    fn started(&mut self, _: &mut SyncContext<Self>) {
        let topic = self.topic.clone();
        match self.server {
            None => {
                let server = self.server.take();
                self.server = Some(NatsServer::connect(topic).unwrap());
            }
            _ => {}
        }
        // let fut = async move {
        //     cmd
        //         .query_async(&mut con)
        //         .await
        // };
        // Box::pin(fut)

        info!("Nats Actor started up");
    }
    fn stopped(&mut self, _: &mut SyncContext<Self>) {
        info!("Nats Actor Shutdown");
        System::current().stop();
    }
}

impl Handler<NatsTask> for NatsActor {
    type Result = ();
    fn handle(&mut self, _: NatsTask, _: &mut SyncContext<Self>){
        info!("Start Handle Nats Task");
        thread::sleep(time::Duration::new(4,0));
        info!("Finished Nats Task");
    }
}