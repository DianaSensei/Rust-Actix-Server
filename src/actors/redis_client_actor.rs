use std::borrow::BorrowMut;
use redis::{Client};
use redis::aio::MultiplexedConnection;
use actix::prelude::*;

struct RedisActor {
    conn: MultiplexedConnection,
}

impl RedisActor {
    pub async fn new(redis_url: &'static str) -> Self {
        let client = Client::open(redis_url).unwrap();// not recommended
        let conn = client.get_multiplexed_async_connection().await.unwrap();
        RedisActor { conn }
    }
}

#[derive(Message, Debug)]
#[rtype(result = "Result<Option<String>, redis::RedisError>")]
struct InfoCommand;

impl Handler<InfoCommand> for RedisActor {
    type Result = ResponseFuture<Result<Option<String>, redis::RedisError>>;

    fn handle(&mut self, _msg: InfoCommand, _: &mut Self::Context) -> Self::Result {
        let mut con = self.conn.borrow_mut();
        let fut = async move {
            redis::cmd("INFO").query_async(&mut con).await
        };
        Box::pin(fut)
    }
}


impl Actor for RedisActor {
    type Context = Context<Self>;
}