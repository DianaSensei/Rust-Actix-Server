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

use opentelemetry::trace::TraceContextExt;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields, FormattedFields};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::registry::LookupSpan;

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
    let file_appender = tracing_appender::rolling::never("./logs", "app.log");
    let (non_blocking_file, _guard_file) = tracing_appender::non_blocking(file_appender);

    log_config(non_blocking_file);
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

fn log_config(nonblocking_file: tracing_appender::non_blocking::NonBlocking) {
    use crate::settings::SETTINGS;
    use opentelemetry::sdk::propagation::TraceContextPropagator;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    dotenv::dotenv().ok();

    // std::env::set_var(
    //     "RUST_LOG",
    //     "debug, actix_web=debug,actix_server=debug,actix_http=info",
    // );
    std::env::set_var("RUST_LOG_STYLE", "always");
    std::env::set_var("RUST_BACKTRACE", "full"); // debug verbose mode

    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_agent_endpoint(&SETTINGS.tracer.jaeger.url)
        .with_service_name(&SETTINGS.cargo_pkg_name)
        .install_batch(opentelemetry::runtime::TokioCurrentThread)
        .expect("Failed to install OpenTelemetry tracer.");

    let roll = tracing_subscriber::fmt::layer()
        .event_format(TLog::new())
        .with_writer(nonblocking_file);

    let fmt = tracing_subscriber::fmt::layer()
        .event_format(TLog::new());

    let log_level = tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info, actix_web=info,actix_server=info,actix_http=info".into()),
    );

    tracing_subscriber::registry()
        .with(log_level)
        .with(roll)
        .with(fmt)
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(tracing_bunyan_formatter::JsonStorageLayer)
        .try_init()
        .expect("Unable to install global subscriber");
}

#[derive(Debug)]
pub struct TLog;

impl TLog {
    fn new() -> Self {
        TLog {}
    }
}

impl<S, N> FormatEvent<S, N> for TLog
    where
        S: tracing::Subscriber + for<'a> LookupSpan<'a>,
        N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {

        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
        write!(writer, "{}", timestamp)?;

        let level = event.metadata().level();
        // Format Log Level
        match *level {
            tracing::Level::TRACE => write!(writer, " {}", ansi_term::Colour::Purple.bold().paint(level.as_str())),
            tracing::Level::DEBUG => write!(writer, " {}", ansi_term::Colour::Blue.bold().paint(level.as_str())),
            tracing::Level::INFO => write!(writer, " {}", ansi_term::Colour::Green.bold().paint(level.as_str())),
            tracing::Level::WARN => write!(writer, " {}", ansi_term::Colour::Yellow.bold().paint(level.as_str())),
            tracing::Level::ERROR => write!(writer, " {}", ansi_term::Colour::Red.bold().paint(level.as_str())),
        }?;

        // Decorate Span info
        let ctx1 = opentelemetry::Context::current();
        let trace_id = ctx1.span().span_context().trace_id();
        let span_id = ctx1.span().span_context().span_id();
        write!(writer, " [{:x},{:x}]", trace_id, span_id)?;

        // get some process information
        let pid = std::process::id();
        let thread = std::thread::current();
        let thread_name_op = thread.name();
        match thread_name_op {
            None => {
                write!(writer, " [{}, ]", pid)?;
            }
            Some(thread_name) => {
                write!(writer, " [{},{:?}]", pid, thread_name)?;
            }
        }


        // Format target
        let target = event..target();
        write!(writer, " [{}]", ansi_term::Colour::Blue.bold().paint(target))?;

        // Format Module
        let module = event.metadata().module_path();
        match module {
            None => {
                write!(writer, " []: ")?;
            }
            Some(modules) => {
              let module_split = modules.split("::");
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

                write!(writer, " {}: ", ansi_term::Colour::Yellow.bold().paint(module_short))?;
            }
        }


        ctx.field_format().format_fields(writer.by_ref(), event)?;

        //     // now, we're printing the span context into brackets of `[]`, which glog parsers ignore.
        let leaf = ctx.lookup_current();

        if let Some(leaf) = leaf {
            // write the opening brackets
            write!(writer, "[")?;

            // Write spans and fields of each span
            let mut iter = leaf.scope().from_root();
            let mut span = iter.next().expect(
                "Unable to get the next item in the iterator; this should not be possible.",
            );
            loop {
                let ext = span.extensions();
                let fields = &ext
                    .get::<FormattedFields<N>>()
                    .expect("will never be `None`");

                let fields = if !fields.is_empty() {
                    Some(fields.as_str())
                } else {
                    None
                };


                let bold = ansi_term::Style::new().bold();
                write!(writer, " {} ", bold.paint(span.name()))?;
                let italic = ansi_term::Style::new().italic();
                if let Some(fields) = fields {
                    write!(writer, " {{{}}} ", italic.paint(fields))?;
                };

                drop(ext);
                match iter.next() {
                    // if there's more, add a space.
                    Some(next) => {
                        write!(writer, ", ")?;
                        span = next;
                    }
                    // if there's nothing there, close.
                    None => break,
                }
            }
            write!(writer, "] ")?;
        }

        writeln!(writer)
    }
}