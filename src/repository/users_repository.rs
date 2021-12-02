use crate::model::domain::pagination::Pagination;
use crate::model::domain::schema::users;
use crate::model::domain::users::{NewUser, User};
use crate::model::response::page_response::PageResponse;
use diesel::prelude::*;
use crate::services::client::postgres_client_service::DbConnection;

pub fn get_all_users(page: i64, per_page: i64, conn: &DbConnection) -> QueryResult<PageResponse<User>> {
    let query = users::table.filter(users::created_time_utc.is_not_null());

    let total: i64 = query.clone().count().first(conn).unwrap_or(0);

    let users = query
        .select(users::all_columns)
        // cursor pagination by id as page number
        .order(users::id)
        .filter(users::id.ge((page * per_page) as i32))
        .limit(per_page)
        .load::<User>(conn)
        .unwrap_or_default();

    let total_pages = (total as f64 / per_page as f64).ceil() as u32;

    let page_info = Pagination {
        number_of_elements: users.len(),
        page: page as u32,
        page_size: per_page as u32,
        total_pages,
        total_elements: total as usize,
    };

    Ok(PageResponse {
        data: users,
        page_info,
    })
}

pub fn create_user(user: NewUser, conn: &DbConnection) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)
}

pub fn update_user(user: User, conn: &DbConnection) -> QueryResult<User> {
    diesel::update(users::table).set(&user).get_result(conn)
}
