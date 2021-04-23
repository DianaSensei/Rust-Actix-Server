#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate log;

#[allow(dead_code)]
mod config;
#[allow(dead_code)]
mod errors;
#[allow(dead_code)]
mod services;
#[allow(dead_code)]
mod controllers;
#[allow(dead_code)]
mod lib;
#[allow(dead_code)]
mod model;
#[allow(dead_code)]
mod core;
#[allow(dead_code)]
mod actors;
#[allow(dead_code)]
mod utils;

use actix::prelude::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // use crate::lib::{nats_broker::*, redis_db::*};
    // use crate::services::nats_server;
    // use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
    use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
    use actix_files::Files;
    setup_log();

    // let mut builder =
    //     SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();
    // let sys = actix::System::new("nats");
    let natActorAddr = SyncArbiter::start(1, actors::nats_actor::NatsActor::default);
    let mut server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            // .data(redis_fac.clone()) //Use Redis
            // .data(nats_fac.clone()) //Use Nats
            .wrap(actix_web::middleware::Logger::default())
            .data(
                actix_web::web::JsonConfig::default()
                    .limit(4096)
                    .error_handler(|err, _req| {
                        println!("Parse Json fail!: {:?}", err);
                        actix_web::error::InternalError::from_response(
                            err,
                            actix_web::HttpResponse::BadRequest().finish(),
                        )
                            .into()
                    }),
            )
            .wrap(ErrorHandlers::new().handler(
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                |mut res| {
                    res.response_mut().headers_mut().insert(
                        actix_web::http::header::CONTENT_TYPE,
                        actix_web::http::HeaderValue::from_static("Error"),
                    );
                    dbg!("ErrorHandlers detect!");
                    Ok(ErrorHandlerResponse::Response(res))
                },
            ))
            // .configure(app::routes::init_route)
            .service(Files::new("/images", "static/images/").show_files_listing())
            .default_service(actix_web::web::route().to(actix_web::HttpResponse::MethodNotAllowed))
    });

    // server = if let Some(l) = listenfd::ListenFd::from_env().take_tcp_listener(0)? {
        // server.listen(l)?
    // } else {
        // server.bind_openssl(&config::CONFIG.server, builder)?
    server.bind(&config::CONFIG.server).unwrap().run().await
    // };
    // server.run().await
}

fn setup_log() {
    use std::io::Write;
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info, actix_web=info, actix_server=info");
    std::env::set_var("RUST_LOG_STYLE", "always");
    // std::env::set_var("RUST_BACKTRACE", "full"); // debug verbose mode
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let module = record.module_path().unwrap();
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let mut level_style = buf.default_level_style(record.level());
            level_style.set_bold(true).set_intense(true);
            writeln!(buf, "[{}][{:?}-{}][{}][{}] {}",
                     timestamp,
                     std::thread::current().id(),
                     std::process::id(),
                     level_style.value(record.level()),
                     module,
                     record.args())
        }).init();
}