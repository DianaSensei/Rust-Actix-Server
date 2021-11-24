use crate::model::enumerate::response::ServerResponse;
use crate::model::request::page_request::PageRequest;
use crate::repository::users_repository;
use actix_web::{guard, web, HttpResponse, Scope};
use validator::Validate;

pub fn router() -> Scope {
    web::scope("/api/v1/users")
        .guard(guard::Header("content-type", "application/json"))
        .service(
            web::resource("")
                .route(web::get().to(get_all_users))
        )
    // .service(
    //     web::resource("")
    //         .route(web::get().to(products::get_products))
    //         .route(web::post().to(products::add_product)),
    // )
    // .service(
    //     web::scope("/{product_id}")
    //         .service(
    //             web::resource("")
    //                 .route(web::get().to(products::get_product_detail))
    //                 .route(web::delete().to(products::remove_product)),
    //         )
    //         .service(
    //             web::scope("/parts")
    //                 .service(
    //                     web::resource("")
    //                         .route(web::get().to(parts::get_parts))
    //                         .route(web::post().to(parts::add_part)),
    //                 )
    //                 .service(
    //                     web::resource("/{part_id}")
    //                         .route(web::get().to(parts::get_part_detail))
    //                         .route(web::delete().to(parts::remove_part)),
    //                 ),
    //         ),
    // )
}

async fn get_all_users(pagination: web::Query<PageRequest>) -> HttpResponse {
    if let Err(e) = pagination.validate() {
        return ServerResponse::<i64>::from(e).into();
    }

    let result = users_repository::get_all_users(pagination.page, pagination.page_size).await;
    info!("Response: {}", result);
    ServerResponse::Success(result).into()
}
