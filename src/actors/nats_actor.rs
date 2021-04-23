use actix::prelude::*;
use std::{thread, time};

#[derive(Default)]
pub struct NatsTask;
impl Message for NatsTask {
    type Result = ();
}

#[derive(Default)]
pub struct NatsActor;

impl Actor for NatsActor {
    type Context = Context<Self>;
    fn started(&mut self, _: &mut Context<Self>) {
        info!("Nats Actor started up");
    }
}

impl Handler<NatsTask> for NatsActor {
    type Result = ();
    fn handle(&mut self, _: NatsTask, _: &mut Context<Self>){
        info!("Start Handle Nats Task");
        thread::sleep(time::Duration::new(4,0));
        info!("Finished Nats Task");
    }
}