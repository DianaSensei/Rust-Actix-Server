#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate strum;

use crate::settings::SETTINGS;

#[allow(dead_code)]
mod config;
#[allow(dead_code)]
mod controllers;
#[allow(dead_code)]
mod middlewares;
#[allow(dead_code)]
mod model;
#[allow(dead_code)]
mod repositories;
#[allow(dead_code)]
mod services;
#[allow(dead_code)]
mod settings;
#[allow(dead_code)]
mod utils;

#[actix_web::main]
async fn main() {
    log_config();
    log_credits();

    // Create clients connections
    services::clients::get_kafka_connection().await;
    services::clients::get_nats_connection().await;
    services::clients::get_redis_connection().await;
    services::clients::get_smtp_connection().await;

    utils::hasher::get_argon2_hasher();
    // Create Database connection and run migration
    services::clients::postgres_client_service::init_and_run_migration();

    // Start Consumers
    services::start_registered_consumer().await;

    // Start Web server
    services::start_web_service().await;
}

fn log_credits() {
    use crate::utils::project_profile::get_profile;

    info!("-------- START CREDITS ------------");
    info!("Author: {}", env!("CARGO_PKG_AUTHORS"));
    info!("App Name: {}", env!("CARGO_PKG_NAME"));
    info!("Home Page: {}", env!("CARGO_PKG_HOMEPAGE"));
    info!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
    info!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
    info!("App Version: {}", env!("CARGO_PKG_VERSION"));
    info!("Environment Profile: {}", get_profile().to_string());
    info!("-------- END CREDITS --------------");
}

fn log_config() {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "trace, actix_web=info,actix_server=info,actix_http=info",
    );
    std::env::set_var("RUST_LOG_STYLE", "always");
    std::env::set_var("RUST_BACKTRACE", "full"); // debug verbose mode

    use opentelemetry::sdk::propagation::TraceContextPropagator;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::fmt::writer::MakeWriterExt;

    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_agent_endpoint(&SETTINGS.tracer.jaeger.url)
        .with_service_name(&SETTINGS.cargo_pkg_name)
        .install_batch(opentelemetry::runtime::TokioCurrentThread)
        .expect("Failed to install OpenTelemetry tracer.");

    let console = tracing_subscriber::fmt::layer()
        .compact()
        .with_writer(std::io::stdout.with_max_level(tracing::Level::DEBUG))
        .with_ansi(true);

    let file_appender = tracing_appender::rolling::never("./logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let roll = tracing_subscriber::fmt::layer()
        .compact()
        .with_writer(non_blocking)
        .with_ansi(true);

    tracing_subscriber::registry()
        .with(console)
        .with(roll)
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(tracing_bunyan_formatter::JsonStorageLayer)
        .try_init().expect("Unable to install global subscriber");
}