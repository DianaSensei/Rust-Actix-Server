use crate::controllers;
use crate::middlewares;
use crate::settings;
use actix_cors::Cors;
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
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;

#[allow(unused_must_use)]
pub async fn start_web_service() {
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
            .wrap(RequestMetrics::new(
                opentelemetry::global::meter(&settings::SETTINGS.cargo_pkg_name),
                Some(|req: &actix_web::dev::ServiceRequest| {
                    req.path() == "/metrics" && req.method() == actix_web::http::Method::GET
                }),
                Some( opentelemetry_prometheus::exporter().init()),
            ))
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

    if settings::SETTINGS.server.tls_enable {
        let config = load_tls_config();
        server = match ListenFd::from_env().take_tcp_listener(0).unwrap() {
            Some(listener) => server.listen_rustls(listener, config).unwrap(),
            None => server
                .bind_rustls(&settings::SETTINGS.server.listen_url, config)
                .unwrap(),
        };
    } else {
        server = match ListenFd::from_env().take_tcp_listener(0).unwrap() {
            Some(listener) => server.listen(listener).unwrap(),
            None => server.bind(&settings::SETTINGS.server.listen_url).unwrap(),
        };
    }

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
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => HttpResponse::UnprocessableEntity().body(detail),
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}

fn default_error_handler<B>(res: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    error!("Default ErrorHandlers detected!");
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

fn load_tls_config() -> ServerConfig {
    let cert_file = &mut BufReader::new(File::open(&settings::SETTINGS.server.tls_cert_file_path).unwrap());
    let key_file = &mut BufReader::new(File::open(&settings::SETTINGS.server.tls_key_file_path).unwrap());
    let cert_chain_byte = certs(cert_file).unwrap();
    let mut keys_byte = pkcs8_private_keys(key_file).unwrap();
    if keys_byte.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }
    let mut cert_chain = Vec::new();
    for cert in cert_chain_byte {
        cert_chain.push(Certificate(cert));
    }

    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, PrivateKey(keys_byte.remove(0)))
        .expect("bad certificate/key")
}
