use super::schema::users;
use crate::model::enumerate::user_status::UserStatus;
use chrono::NaiveDateTime;
use crate::model::domain::language::Language;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "users"]
// #[belongs_to(User)]
pub struct User {
    pub id: String,
    pub email: String,
    pub user_name: Option<String>,
    pub hashed_password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    // pub status: UserStatus,
    // pub language: Language,
    pub role: String,
    pub created_by: String,
    pub created_time_utc: NaiveDateTime,
    pub updated_by: String,
    pub updated_time_utc: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateUser {
    // #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    // #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub role: Option<String>,
    #[validate(phone(message = "phone_number not valid"))]
    // #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,
}
