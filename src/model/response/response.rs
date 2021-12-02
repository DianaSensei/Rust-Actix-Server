use actix_web::error::BlockingError;
use actix_web::HttpResponse;
use diesel::result::Error as DieselError;
use serde::Serialize;
use serde_json::{Map as JsonMap, Value as Json};
use std::fmt;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use validator::ValidationErrors;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Response<T> {
    pub code: u16,
    pub message: &'static str,
    pub data: T,
}

impl<T> fmt::Display for Response<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ \"code\": {}, \"message\": {}, \"data\": {}}})",
            self.code, self.message, self.data
        )
    }
}

pub struct ErrResponse(HttpResponse);

impl From<DieselError> for ErrResponse {
    fn from(error: DieselError) -> Self {
        let res = match error {
            DieselError::DatabaseError(_, _err) => HttpResponse::InternalServerError().finish(),
            DieselError::NotFound => HttpResponse::NotFound().finish(),
            err => {
                error!("Unknown Diesel error: {}", err);
                HttpResponse::InternalServerError().finish()
            }
        };
        ErrResponse(res)
    }
}

impl<E> From<BlockingError<E>> for ErrResponse
where
    E: Debug,
{
    fn from(error: BlockingError<E>) -> Self {
        error!("Blocking Error: {:?}", error);
        ErrResponse(HttpResponse::InternalServerError().finish())
    }
}

impl From<ValidationErrors> for ErrResponse {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();
        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<Json> = errors.iter().map(|error| json!(error)).collect();
            err_map.insert(field.to_string(), json!(errors));
        }
        ErrResponse(HttpResponse::BadRequest().json(json!({ "fields": json!(err_map) })))
    }
}

impl Into<HttpResponse> for ErrResponse {
    fn into(self) -> HttpResponse {
        self.0
    }
}

impl Deref for ErrResponse {
    type Target = HttpResponse;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ErrResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
