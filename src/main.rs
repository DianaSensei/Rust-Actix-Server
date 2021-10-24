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


// #[macro_use]
// extern crate diesel;

#[allow(dead_code)]
mod config;
#[allow(dead_code)]
mod controllers;
#[allow(dead_code)]
mod middleware;
#[allow(dead_code)]
mod actors;
#[allow(dead_code)]
mod services;

#[actix_rt::main]
async fn main() {
    log_config();
    // let natActorAddr = SyncArbiter::start(1, actors::nats_actor::NatsActor::);
    // let exe = async {
    //     let natActorAddr = actors::nats_listener_actor::NatsActor::new();
    // };
    // actix::Arbiter::spawn(async move {
    //     actors::nats_listener_actor::NatsActor.start();
    // }, ());

    services::start_web_service().await;
    services::start_registered_consumer().await;
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