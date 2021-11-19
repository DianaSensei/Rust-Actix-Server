use std::ops::{DerefMut};
use crate::model::domain::schema::users;
use crate::model::domain::users::User;
use crate::model::domain::pagination::Paginate;
use crate::model::response::page_result::PageResponse;
use crate::services::client::get_database_connection;
use diesel::prelude::*;

pub async fn get_all_users(page: i64, per_page: i64) -> PageResponse<User>{
    let mut conn = get_database_connection();

    let query = users::table
        .order(users::created_time_utc.desc())
        .filter(users::created_time_utc.is_not_null())
        .select(users::all_columns)
        .paginate(page)
        .per_page(per_page);

    let (users_found, pagination) = query.load_and_count_pages::<User>(conn.deref_mut()).unwrap();

    PageResponse {
        data: users_found,
        pageable: pagination
    }
}