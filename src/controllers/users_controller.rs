use crate::model::domains::users::{NewUser, ResponseUser};
use crate::model::enumerates::logic_code::StatusCode;
use crate::model::enumerates::user_role::UserRole;
use crate::model::enumerates::user_status::UserStatus;
use crate::model::requests::language_request::LanguageRequest;
use crate::model::requests::page_request::PageRequest;
use crate::model::requests::user::register_request::Register;
use crate::model::responses::page_response::PageResponse;
use crate::model::responses::response::{ErrResponse, Response};
use crate::repositories::users_repository;
use crate::services::clients::get_database_connection;
use crate::utils::hasher::get_argon2_hasher;
use crate::utils::translations::*;
use actix_web::{guard, web, HttpResponse, Scope};
use chrono::Utc;
use diesel::connection::TransactionManager;
use diesel::Connection;
use rosetta_i18n::Language;
use rosetta_i18n::LanguageId;
use validator::Validate;

pub fn router() -> Scope {
    web::scope("/api/v1/users")
        .guard(guard::Header("content-type", "application/json"))
        .service(
            web::resource("")
                .route(web::get().to(get_all_users))
                .route(web::post().to(create_user)),
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

#[tracing::instrument]
#[allow(clippy::async_yields_async)]
async fn create_user(
    register: web::Json<Register>,
    language: web::Query<LanguageRequest>,
) -> HttpResponse {
    // Do validate requests
    if let Err(e) = register.validate() {
        return ErrResponse::from(e).into();
    }

    // Detect Language Mapper
    let lang = Lang::from_language_id(&LanguageId::new(language.value.as_str()))
        .unwrap_or_else(Lang::fallback);

    // Do Hash Password with Argon2 Algorithm
    let password = register.password.clone().unwrap();
    let hashed_password =
        web::block(move || get_argon2_hasher().hash(password.as_str()).unwrap()).await;

    if let Err(e) = hashed_password {
        error!("Hash password error: {:?}", e);
        return ErrResponse::from(e).into();
    }

    let email = register.email.clone().unwrap();
    let result = web::block(move || {
        let conn = get_database_connection();
        let user = NewUser {
            email,
            user_name: None,
            hashed_password: hashed_password.unwrap(),
            first_name: None,
            last_name: None,
            phone_number: None,
            status: UserStatus::Inactive,
            role: UserRole::User,
            created_by: "REGISTER".to_string(),
            created_time_utc: Utc::now().naive_utc(),
            updated_by: "REGISTER".to_string(),
            updated_time_utc: Utc::now().naive_utc(),
        };
        users_repository::create_user(user, &conn).unwrap()
    })
    .await;

    if let Err(e) = result {
        error!("Save new User error: {:?}", e);
        return ErrResponse::from(e).into();
    }

    let user_response = ResponseUser::from(result.unwrap());
    info!("Response: {}", user_response);
    HttpResponse::Ok().json(Response {
        code: 0,
        message: lang.success(),
        data: user_response,
    })
}

#[tracing::instrument]
#[allow(clippy::async_yields_async)]
async fn get_all_users(
    pagination: web::Query<PageRequest>,
    language: web::Query<LanguageRequest>,
) -> HttpResponse {
    // Do validate requests
    if let Err(e) = pagination.validate() {
        return ErrResponse::from(e).into();
    }

    // Detect Language Mapper
    let lang = Lang::from_language_id(&LanguageId::new(language.value.as_str()))
        .unwrap_or_else(Lang::fallback);

    let result = web::block(move || {
        let conn = get_database_connection();
        let _ = conn.transaction_manager().begin_transaction(&conn);
        info!("Begin transaction");
        let res = users_repository::get_all_users(pagination.page, pagination.page_size, &conn);

        if let Err(e) = res {
            let _ = conn.transaction_manager().rollback_transaction(&conn);
            info!("Rollback transaction");
            return Err(e);
        }
        let _ = conn.transaction_manager().commit_transaction(&conn);
        info!("Committed transaction");

        res
    })
    .await;

    if let Err(e) = result {
        return ErrResponse::from(e).into();
    }

    if let Err(e) = result.as_ref().unwrap() {
        error!("Get User error: {:?}", e);
        return ErrResponse::from(e).into();
    }

    let page_response: PageResponse<ResponseUser> = result.unwrap().unwrap().into();
    info!("Response: {}", page_response);

    HttpResponse::Ok().json(Response {
        code: StatusCode::SUCCESS.into(),
        message: lang.success(),
        data: page_response,
    })
}
