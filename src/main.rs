#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate strum;

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
    use crate::settings::SETTINGS;
    use env_logger::fmt::Color;
    use log::Level;
    use opentelemetry::sdk::propagation::TraceContextPropagator;
    use opentelemetry::trace::TraceContextExt;
    use std::io::Write;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    dotenv::dotenv().ok();
    // std::env::set_var(
    //     "RUST_LOG",
    //     "info, actix_web=info,actix_server=info,actix_http=info",
    // );
    std::env::set_var("RUST_LOG_STYLE", "always");
    std::env::set_var("RUST_BACKTRACE", "full"); // debug verbose mode

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|writer, record| {
            // Format Local TimeStamp
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            write!(writer, "{}", timestamp)?;

            // Format Log Level
            let mut level_style = writer.default_level_style(record.level());
            level_style.set_bold(true);
            let level = match record.level() {
                Level::Warn | Level::Info => record.level().as_str().to_owned() + " ",
                _ => record.level().to_string(),
            };
            write!(writer, " {}", level_style.value(level))?;

            // Decorate Span info
            let ctx = opentelemetry::Context::current();
            let trace_id = match ctx.span().span_context().trace_id()
                == opentelemetry::trace::TraceId::invalid()
            {
                false => ctx.span().span_context().trace_id().to_hex(),
                true => "".to_string(),
            };
            let span_id = match ctx.span().span_context().span_id()
                == opentelemetry::trace::SpanId::invalid()
            {
                false => ctx.span().span_context().span_id().to_hex(),
                true => "".to_string(),
            };
            write!(writer, " [{},{}]", trace_id, span_id)?;

            // Format target
            let mut target_style = writer.style();
            target_style.set_color(Color::Blue).set_bold(true);
            write!(writer, " [{}]", target_style.value(record.target()))?;

            // Format Module
            let module_split = record.module_path().unwrap().split("::");
            let count = module_split.clone().count();
            let mut module_short = String::new();
            for (pos, module) in module_split.enumerate() {
                if pos == count - 1 {
                    module_short.push_str(module);
                } else {
                    module_short.push(module.chars().next().unwrap());
                    module_short.push_str(".");
                }
            }
            let mut module_style = writer.style();
            module_style.set_color(Color::Yellow).set_bold(true);
            write!(writer, " {}", module_style.value(module_short))?;

            write!(writer, ": {}", record.args())?;
            writeln!(writer)
        })
        .init();

    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_agent_endpoint(&SETTINGS.tracer.jaeger.url)
        .with_service_name(&SETTINGS.cargo_pkg_name)
        .install_batch(opentelemetry::runtime::TokioCurrentThread)
        .expect("Failed to install OpenTelemetry tracer.");

    let file_appender = tracing_appender::rolling::never("./logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let roll = tracing_subscriber::fmt::layer()
        .compact()
        .with_writer(non_blocking)
        .with_ansi(true);

    tracing_subscriber::registry()
        .with(roll)
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(tracing_bunyan_formatter::JsonStorageLayer)
        .try_init()
        .expect("Unable to install global subscriber");
}
