// use diesel::Insertable;
use diesel::Identifiable;
use serde::{Deserialize, Serialize};
use super::schema::users;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "users"]
// #[belongs_to(User)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub role: String,
    pub created_by: String,
    pub created_time_dt: NaiveDateTime,
    pub updated_by: String,
    pub updated_time_dt: NaiveDateTime,
    #[diesel(deserialize_as = "Option<NaiveDateTime>")]
    pub pub_status: Status
}

pub enum Status {
    Draft,
    Published { at: NaiveDateTime },
}

impl Into<Status> for Option<NaiveDateTime> {
    fn into(self) -> Status {
        match self {
            None => Status::Draft,
            Some(at) => Status::Published { at },
        }
    }
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
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