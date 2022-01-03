use crate::model::responses::health_response::HealthResponse;
use actix_web::{web, HttpResponse, Scope};

pub fn router() -> Scope {
    web::scope("/api/v1/health").service(web::resource("").to(get_health))
}

#[tracing::instrument]
#[allow(clippy::async_yields_async)]
async fn get_health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "Ok".to_owned(),
        version: "Cargo Version: ".to_string() + env!("CARGO_PKG_VERSION"),
    })
}
