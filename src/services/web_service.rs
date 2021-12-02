use crate::config;
use crate::controllers;
use crate::middleware;
use actix_cors::Cors;
use actix_web::{
    error, http,
    middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers},
    middleware::Compress,
    web, App, HttpRequest, HttpResponse, HttpServer,
};
use listenfd::ListenFd;

pub async fn start_web_service() {
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            // .wrap(actix_session::CookieSession::signed(&[0; 32]).secure(false))
            .wrap(middleware::LoggingRequestMiddleware)
            // Cors Config
            .wrap(cors_config())
            // Json Handler Config
            .data(json_config())
            // Default Error Handler
            .wrap(
                ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, |res| {
                    error!("Default ErrorHandlers detected!");
                    Ok(ErrorHandlerResponse::Response(res))
                }),
            )
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
