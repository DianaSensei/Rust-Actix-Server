use super::schema::users;
use crate::model::enumerate::user_role::UserRole;
use crate::model::enumerate::user_status::UserStatus;
use crate::model::response::page_response::PageResponse;
use chrono::NaiveDateTime;
use itertools::Itertools;
use std::fmt;
use std::fmt::Formatter;

#[derive(Serialize, Deserialize, Queryable, Identifiable, AsChangeset, Debug, Clone, PartialEq)]
#[table_name = "users"]
// #[belongs_to(User)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub user_name: Option<String>,
    pub hashed_password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub status: UserStatus,
    pub role: UserRole,
    pub created_by: String,
    pub created_time_utc: NaiveDateTime,
    pub updated_by: String,
    pub updated_time_utc: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Debug, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub user_name: Option<String>,
    pub hashed_password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub status: UserStatus,
    pub role: UserRole,
    pub created_by: String,
    pub created_time_utc: NaiveDateTime,
    pub updated_by: String,
    pub updated_time_utc: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUser {
    pub email: String,
    pub user_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub status: UserStatus,
    pub role: UserRole,
    pub created_by: String,
    pub created_time_utc: NaiveDateTime,
    pub updated_by: String,
    pub updated_time_utc: NaiveDateTime,
}

impl From<User> for ResponseUser {
    fn from(user: User) -> Self {
        ResponseUser {
            email: user.email.clone(),
            user_name: user.user_name.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            phone_number: user.phone_number.clone(),
            status: user.status,
            role: user.role,
            created_by: user.created_by.clone(),
            created_time_utc: user.created_time_utc,
            updated_by: user.updated_by.clone(),
            updated_time_utc: user.updated_time_utc,
        }
    }
}

impl From<&User> for ResponseUser {
    fn from(user: &User) -> Self {
        (*user).clone().into()
    }
}

impl From<PageResponse<User>> for PageResponse<ResponseUser> {
    fn from(page_user: PageResponse<User>) -> Self {
        PageResponse {
            page_info: page_user.page_info,
            data: page_user
                .data
                .iter()
                .map_into::<ResponseUser>()
                .collect_vec(),
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut processed_data = self.clone();
        processed_data.hashed_password = String::from("********");
        write!(f, "{}", serde_json::to_string(&processed_data).unwrap())
    }
}

impl fmt::Display for ResponseUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
