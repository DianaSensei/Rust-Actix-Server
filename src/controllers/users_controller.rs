use crate::model::enumerate::response::ServerResponse;
use crate::model::request::page_request::PageRequest;
use crate::repository::users_repository;
use actix_web::{guard, web, HttpResponse};
use validator::Validate;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .guard(guard::Header("content-type", "application/json"))
            .service(web::resource("users").route(web::get().to(get_all_users))),
    );
}

async fn get_all_users(pagination: web::Query<PageRequest>) -> HttpResponse {
    if let Err(e) = pagination.validate() {
        return ServerResponse::<i64>::from(e).into();
    }

    let result = users_repository::get_all_users(pagination.page, pagination.pagesize).await;
    info!("Response: {}", result);
    ServerResponse::Success(result).into()
}
