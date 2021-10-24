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

use actix_cors::Cors;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::web::JsonConfig;
// #[macro_use]
// extern crate diesel;

#[allow(dead_code)]
mod config;
#[allow(dead_code)]
mod controllers;
#[allow(dead_code)]
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // use actix::SyncArbiter;
    // use actix::prelude::*;
    log_config();
    // let natActorAddr = SyncArbiter::start(1, actors::nats_actor::NatsActor::);
    // let exe = async {
    //     let natActorAddr = actors::nats_actor::NatsActor.start();
    // };
    // actix_rt::Arbiter::spawn(async move {
    //     actors::nats_actor::NatsActor.start();
    // });
    // natActorAddr.do_send(actors::nats_actor::NatsTask{});
    // natActorAddr.do_send(actors::nats_actor::NatsTask{});
    let mut server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Compress::default())
            // .wrap(actix_session::CookieSession::signed(&[0; 32]).secure(false))
            .wrap(middleware::LoggingRequestMiddleware)
            // Cors Config
            .wrap(cors_config())
            // .data(natActorAddr)
            // Json Handler Config
            .data(json_config())
            // Default Error Handler
            .wrap(ErrorHandlers::new().handler(
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                |res| {
                    error!("Default ErrorHandlers detected!");
                    Ok(ErrorHandlerResponse::Response(res))
                })
            )
            // Endpoint Config
            .configure(controllers::routes::init_route)
            // Default EndPoint
            .default_service(actix_web::web::route().to(actix_web::HttpResponse::MethodNotAllowed))
    });

    server = if let Some(l) = listenfd::ListenFd::from_env().take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&config::CONFIG.server)?
    };
    server.run().await
}

fn log_config() {
    use std::io::Write;
    use env_logger::fmt::Color;
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "info, actix_web=info, actix_server=info");
    std::env::set_var("RUST_LOG_STYLE", "always");
    // std::env::set_var("RUST_BACKTRACE", "full"); // debug verbose mode

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            // Format Module
            let module_split = record.module_path().unwrap().split("::");
            let count = module_split.clone().count();
            let mut module_short = String::new();
            for (pos, module) in module_split.enumerate() {
                if pos == count - 1 {
                    module_short.push_str(module);
                } else {
                    module_short.push(module.chars().next().unwrap());
                    module_short.push_str("::");
                }
            }
            let mut module_style = buf.style();
            module_style.set_color(Color::Magenta);

            // Format Local TimeStamp
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");

            // Format Log Level
            let mut level_style = buf.default_level_style(record.level());
            level_style.set_bold(true).set_intense(true);

            // Write Log Format
            writeln!(buf, "{} {} [{:?}-{}][{}]: {}",
                     timestamp,
                     level_style.value(record.level()),
                     std::thread::current().id(),
                     std::process::id(),
                     module_style.value(module_short),
                     record.args())
        }).init();
}

fn json_config() -> JsonConfig {
    actix_web::web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            error!("Parse Json fail!: {:?}", err);
            actix_web::error::InternalError::from_response(
                err, actix_web::HttpResponse::BadRequest().finish()
            ).into()
        })
}

fn cors_config() -> Cors {
    use actix_web::http::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};

    Cors::default()
            .send_wildcard()
            .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE, ACCEPT])
            .supports_credentials()
            .max_age(3600)
}