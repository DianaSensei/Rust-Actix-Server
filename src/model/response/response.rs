use crate::model::enumerate::response::ServerResponse;
use actix_web::{error, HttpRequest, HttpResponse};
use diesel::result::Error as DieselError;
use serde::Serialize;
use serde_json::{Map as JsonMap, Value as Json};
use std::fmt;
use validator::ValidationErrors;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Response<T> {
    pub code: u16,
    pub message: String,
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

impl<T> From<DieselError> for ServerResponse<T> {
    fn from(error: DieselError) -> ServerResponse<T> {
        match error {
            DieselError::DatabaseError(_, _err) => ServerResponse::InternalServerError,
            DieselError::NotFound => ServerResponse::NotFound,
            err => {
                error!("Unknown Diesel error: {}", err);
                ServerResponse::InternalServerError
            }
        }
    }
}

impl<T: Serialize> Into<HttpResponse> for ServerResponse<T> {
    fn into(self) -> HttpResponse {
        match self {
            ServerResponse::BadRequest(err) => HttpResponse::BadRequest().json(err),
            ServerResponse::NotFound => HttpResponse::NotFound().finish(),
            ServerResponse::Unauthorized(err) => HttpResponse::Unauthorized().json(err),
            ServerResponse::Conflict => HttpResponse::Conflict().finish(),
            ServerResponse::Forbidden(err) => HttpResponse::Forbidden().json(err),
            ServerResponse::UnprocessableEntity(json) => HttpResponse::BadRequest().json(json),
            ServerResponse::RequestTimeOut => HttpResponse::RequestTimeout().finish(),
            ServerResponse::MethodNotAllowed => HttpResponse::MethodNotAllowed().finish(),
            ServerResponse::BadGateway => HttpResponse::BadGateway().finish(),
            ServerResponse::Success(data) => HttpResponse::Ok().json(data),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}

impl<T> From<ValidationErrors> for ServerResponse<T> {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();
        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<Json> = errors.iter().map(|error| json!(error)).collect();
            err_map.insert(field.to_string(), json!(errors));
        }

        ServerResponse::BadRequest(json!({ "fields": json!(err_map) }))
    }
}

pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}
