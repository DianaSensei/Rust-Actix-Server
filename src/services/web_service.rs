use crate::config;
use crate::controllers;
use crate::middleware;
use actix_cors::Cors;
use actix_web::{
    error, http,
    middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers},
    middleware::Compress,
    web, App, HttpResponse, HttpServer,
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
        .error_handler(|err, req| {
            error!("Parse Json {:?} cause error: {:?}", req, err);
            error::InternalError::from_response(err, HttpResponse::BadRequest().finish()).into()
        })
}

fn cors_config() -> Cors {
    use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

    Cors::default()
        .send_wildcard()
        .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE, ACCEPT])
        .supports_credentials()
        .max_age(3600)
}
