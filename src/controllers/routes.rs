use actix_web::{web, HttpResponse};
pub fn init_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // .guard(guard::Header("content-type", "application/json"))
            .service(web::resource("health").to(get_health))
            // .service(web::resource("set").to(set_health))
            .service(
                web::resource("users")
                    .route(web::get().to(set_health))
                    // .wrap(middleware::read_request_body::Logging)
                    .route(web::post().to(set_health))
                    .route(web::delete().to(set_health))
            )
            .service(
                web::resource("users/{id}")
                    .route(web::get().to(set_health))
                    .route(web::put().to(set_health))
                    .route(web::delete().to(set_health))
                    // .route(web::delete().to(find_delete_user))
            )
            .service(web::resource("nats/users").route(web::post().to(set_health)))
            // .service(web::resource("admin").to(admin))
            // .service(web::resource("auth").to(login)),
    );
}
#[derive(Serialize)]
struct HealthResponse {
    pub status: String,
    pub version: String,
}
use crate::lib::redis_db::*;
use crate::lib::nats_broker::*;

async fn get_health(_pool: web::Data<RedisFactory>,
    _nats_pool: web::Data<NatsConnection>,
) -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "".to_owned(),
        version: "Cargo Version: ".to_string() + env!("CARGO_PKG_VERSION").into(),
    })
}
async fn set_health(pool: web::Data<RedisFactory>) -> HttpResponse {
    let _res = set_str(&pool.pool, "abc", "1234", 0).await.unwrap();
    HttpResponse::Ok().json(HealthResponse {
        status: "Ok".into(),
        version: "Cargo Version: ".to_string() + env!("CARGO_PKG_VERSION").into(),
    })
}
