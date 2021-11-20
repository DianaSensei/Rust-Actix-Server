use crate::model::response::health_response::HealthResponse;
use actix_web::{guard, web, HttpResponse};

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .guard(guard::Header("content-type", "application/json"))
            .service(web::resource("health").to(get_health)),
    );
}

async fn get_health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "Ok".to_owned(),
        version: "Cargo Version: ".to_string() + env!("CARGO_PKG_VERSION").into(),
    })
}
