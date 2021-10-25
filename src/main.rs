#[macro_use]
extern crate serde_derive;
// #[macro_use]
// extern crate serde_json;
// #[macro_use]
// extern crate validator_derive;
#[macro_use]
extern crate log;
// #[macro_use]
// extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

#[allow(dead_code)]
mod config;
#[allow(dead_code)]
mod controllers;
#[allow(dead_code)]
mod middleware;
#[allow(dead_code)]
mod services;
#[allow(dead_code)]
mod utils;
#[allow(dead_code)]
mod model;


#[actix_web::main]
async fn main() {
    log_config();

    services::client::postgres_client_service::init_and_run_migration();
    services::start_registered_consumer().await;
    services::start_web_service().await;
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