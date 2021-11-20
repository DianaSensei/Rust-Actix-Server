use super::schema::users;
use crate::model::domain::language::Language;
use crate::model::enumerate::user_status::UserStatus;
use chrono::NaiveDateTime;
use std::fmt;
use std::fmt::Formatter;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
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
    // pub status: UserStatus,
    // pub language: Language,
    pub role: String,
    pub created_by: String,
    pub created_time_utc: NaiveDateTime,
    pub updated_by: String,
    pub updated_time_utc: NaiveDateTime,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut processed_data = self.clone();
        processed_data.hashed_password = String::from("********");
        write!(f, "{}", serde_json::to_string(&processed_data).unwrap())
    }
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
