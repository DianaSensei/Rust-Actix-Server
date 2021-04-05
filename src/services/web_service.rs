use actix_cors::Cors;
use actix_web::{
    middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers},
    middleware::Compress,
    http,
    web,
    error,
    HttpServer,
    HttpResponse,
    App
};
use crate::middleware;
use crate::controllers;
use crate::config;

pub async fn start_web_service() {
    let _ = HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            // .wrap(actix_session::CookieSession::signed(&[0; 32]).secure(false))
            .wrap(middleware::LoggingRequestMiddleware)
            // Cors Config
            .wrap(cors_config())
            // .data(natActorAddr)
            // Json Handler Config
            .data(json_config())
            // Default Error Handler
            .wrap(ErrorHandlers::new().handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                |res| {
                    error!("Default ErrorHandlers detected!");
                    Ok(ErrorHandlerResponse::Response(res))
                })
            )
            // Endpoint Config
            .configure(controllers::routes::init_route)
            // Default EndPoint
            .default_service(web::route().to(HttpResponse::MethodNotAllowed))
    })
        .bind(&config::CONFIG.server)
        .unwrap()
        .run()
        .await;
}

fn json_config() -> web::JsonConfig {
    web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            error!("Parse Json fail!: {:?}", err);
            error::InternalError::from_response(
                err, HttpResponse::BadRequest().finish()
            ).into()
        })
}

fn cors_config() -> Cors {
    use http::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};

    Cors::default()
        .send_wildcard()
        .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE, ACCEPT])
        .supports_credentials()
        .max_age(3600)
}