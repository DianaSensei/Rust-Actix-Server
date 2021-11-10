use serde_json::Value as Json;
use crate::model::response::response::Response;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ServerResponse<T> {
    #[error("Success")]
    Success(Response<T>),
    #[error("Bad Request")]
    BadRequest(Json),
    #[error("Not Found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized(String),
    #[error("Forbidden")]
    Forbidden(String),
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Method Not Allowed")]
    MethodNotAllowed,
    #[error("Bad Gateway")]
    BadGateway,
    #[error("Resource Exists")]
    Conflict,
    #[error("Unprocessable Entity: {}", _0)]
    UnprocessableEntity(Json),
    #[error("Request Time Out")]
    RequestTimeOut,
}
