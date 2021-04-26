use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct Login {
    #[validate(email(message = "email is not valid"))]
    pub email: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
    #[serde(default)]
    pub remember_me: bool,
}