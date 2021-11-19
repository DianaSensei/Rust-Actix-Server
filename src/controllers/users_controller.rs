use actix_web::{guard, HttpResponse, web};
use crate::model::request::page_request::PageRequest;
use crate::model::response::health_response::HealthResponse;
use crate::repository::users_repository;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .guard(guard::Header("content-type", "application/json"))
            .service(web::resource("users")
                .route(web::get().to(get_all_users))
                .route(web::post().to(get_health))
            )
    );
}

async fn get_health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "Ok1".to_owned(),
        version: "Cargo Version: ".to_string() + env!("CARGO_PKG_VERSION").into(),
    })
}

async fn get_all_users(pagination: web::Query<PageRequest>) -> HttpResponse {
    let result = users_repository::get_all_users(pagination.page, pagination.pagesize).await;
    info!("Response: {}", result);
    HttpResponse::Ok().json(result)
}