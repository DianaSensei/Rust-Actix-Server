use super::lib::*;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct Register {
    #[validate(required, email(message = "email is not valid"))]
    pub email: Option<String>,
    #[validate(
    required,
    length(min = 8, message = "password must be at least 8 characters")
    )]
    pub password: Option<String>,


// #[derive(Serialize, Deserialize, Debug, Validate, Clone)]
// pub struct Confirmation {
//     pub id: String,
//     pub email: String,
//     pub password: String,
//     pub expires_time_dt: DateTime,
// }