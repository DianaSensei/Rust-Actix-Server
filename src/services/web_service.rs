use crate::config;
use crate::controllers;
use crate::middlewares;
use actix_cors::Cors;
use actix_web::dev::Service;
use actix_web::dev::ServiceResponse;
use actix_web::web::Data;
use actix_web::{
    error, http,
    middleware::Compress,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, App, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_opentelemetry::{RequestMetrics, RequestTracing};
use listenfd::ListenFd;
use opentelemetry::trace::TraceContextExt;

#[allow(unused_must_use)]
pub async fn start_web_service() {
    // Start an (optional) otel prometheus metrics pipeline
    let metrics_exporter = opentelemetry_prometheus::exporter().init();
    let request_metrics = RequestMetrics::new(
        opentelemetry::global::meter(&config::CONFIG.cargo_pkg_name),
        Some(|req: &actix_web::dev::ServiceRequest| {
            req.path() == "/metrics" && req.method() == actix_web::http::Method::GET
        }),
        Some(metrics_exporter),
    );

    let mut server = HttpServer::new(move || {
        App::new()
            // Json Handler Config
            .app_data(Data::new(json_config()))
            // Default Error Handler
            .wrap(ErrorHandlers::new().handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                default_error_handler,
            ))
            // .wrap(actix_session::CookieSession::signed(&[0; 32]).secure(false))
            .wrap(middlewares::LoggingRequestMiddleware)
            // Metric Prometheus
            .wrap(request_metrics.clone())
            // MDC decorator
            .wrap_fn(|request, srv| {
                let ctx = opentelemetry::Context::current();
                if ctx.span().span_context().trace_id() != opentelemetry::trace::TraceId::invalid()
                {
                    log_mdc::insert("trace_id", ctx.span().span_context().trace_id().to_hex());
                };
                if ctx.span().span_context().span_id() != opentelemetry::trace::SpanId::invalid() {
                    log_mdc::insert("span_id", ctx.span().span_context().span_id().to_hex());
                };
                actix_web::web::block(move || {
                    log_mdc::insert("trace_id", ctx.span().span_context().trace_id().to_hex());
                    log_mdc::insert("span_id", ctx.span().span_context().span_id().to_hex());
                });
                srv.call(request)
            })
            // Tracing Jeager
            .wrap(RequestTracing::new())
            // Cors Config
            .wrap(cors_config())
            // Compress request
            .wrap(Compress::default())
            // Endpoint Config
            .configure(controllers::router::global_router)
            // Default EndPoint
            .default_service(web::route().to(HttpResponse::NotFound))
    });

    server = match ListenFd::from_env().take_tcp_listener(0).unwrap() {
        Some(listener) => server.listen(listener).unwrap(),
        None => server.bind(&config::CONFIG.server).unwrap(),
    };

    let _ = server.run().await;
    // Ensure all spans have been shipped to Jaeger.
    opentelemetry::global::shutdown_tracer_provider();
}

fn json_config() -> web::JsonConfig {
    web::JsonConfig::default()
        .limit(4096)
        .error_handler(json_error_handler)
}

fn cors_config() -> Cors {
    use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

    Cors::default()
        .send_wildcard()
        .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE, ACCEPT])
        .supports_credentials()
        .max_age(3600)
}

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}

fn default_error_handler<B>(res: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    error!("Default ErrorHandlers detected!");
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}
