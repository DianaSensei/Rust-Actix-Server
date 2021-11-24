use super::schema::users;
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
    pub status: UserStatus,
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
