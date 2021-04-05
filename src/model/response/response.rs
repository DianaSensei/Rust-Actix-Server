use actix_web::{error, error::ResponseError, HttpRequest, HttpResponse};
use actix_web::http::StatusCode;
use serde_json::{Map as JsonMap, Value as Json};
use validator::ValidationErrors;
use std::fmt;
use serde::Serialize;
use diesel::result::Error as DieselError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub code: u16,
    pub message: String,
    pub data: T,
}

impl<T> Response<T> where T: Serialize
{
    pub fn new(code: u16, message: String, data: impl Serialize) -> Response<T> {
        Response { code, message, data}
    }
}

impl fmt::Display for Response<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ code: {}, message: {}, data: {}}})", self.code, self.message, self.data)
    }
}

impl From<DieselError> for Response<T> {
    fn from(error: DieselError) -> Response<T> {
        match error {
            DieselError::DatabaseError(_, err) => Response::new(500, err.message().to_string(), ()),
            DieselError::NotFound => Response::new(404, "Data ".to_string(), ()),
            err => Response::new(500, format!("Unknown Diesel error: {}", err), ()),
        }
    }
}

#[derive(Debug, failure::Fail, PartialEq)]
pub enum ServerResponse {
    #[fail(display = "Success")]
    Success(Response<T>),
    #[fail(display = "Bad Request")]
    BadRequest(Json),
    #[fail(display = "Blocking Error")]
    BlockingError(String),
    #[fail(display = "Not Found")]
    NotFound(String),
    #[fail(display = "Unauthorized")]
    Unauthorized(String),
    #[fail(display = "Forbidden")]
    Forbidden(String),
    #[fail(display = "Pool Error")]
    PoolError(String),
    #[fail(display = "Internal Server Error")]
    InternalServerError,
    #[fail(display = "Method Not Allowed")]
    MethodNotAllowed,
    #[fail(display = "Bad Gateway")]
    BadGateway,
    #[fail(display = "Resource Exists")]
    Conflict,
    #[fail(display = "Database Error")]
    DBError(String),
    #[fail(display = "Unprocessable Entity: {}", _0)]
    UnprocessableEntity(Json),
    #[fail(display = "Time Out")]
    RequestTimeOut,
}
impl fmt::Display for ServerResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name)
    }
}
impl From<ServerResponse> for HttpResponse {
    fn from(res: ServerResponse) -> Self {
        match res {
            ServerResponse::BadRequest(err) => HttpResponse::BadRequest().json(err),
            ServerResponse::NotFound(err) => HttpResponse::NotFound().json(err),
            ServerResponse::Unauthorized(err) =>HttpResponse::Unauthorized().json(err),
            ServerResponse::Conflict => HttpResponse::Conflict().finish(),
            ServerResponse::Forbidden(err) => HttpResponse::Forbidden().json(err),
            ServerResponse::UnprocessableEntity(json) => HttpResponse::BadRequest().json(json),
            ServerResponse::RequestTimeOut => HttpResponse::RequestTimeout().finish(),
            ServerResponse::MethodNotAllowed => HttpResponse::MethodNotAllowed().finish(),
            ServerResponse::BadGateway => HttpResponse::BadGateway().finish(),
            ServerResponse::NoContent => HttpResponse::NoContent().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
impl From<ValidationErrors> for ServerResponse {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();
        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<Json> = errors
                .iter()
                .map(|error| {
                    json!(error.message)
                })
                .collect();
            err_map.insert(field.to_string(), json!(errors));
        }

        ServerResponse::BadRequest(json!(err_map))
    }
}
// pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
//     use actix_web::error::JsonPayloadError;
//
//     let detail = err.to_string();
//     let resp = match &err {
//         JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
//         JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
//             HttpResponse::UnprocessableEntity().body(detail)
//         }
//         _ => HttpResponse::BadRequest().body(detail),
//     };
//     error::InternalError::from_response(err, resp).into()
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct ErrorResponse {
//     errors: Vec<String>,
// }
//
// impl From<&String> for ErrorResponse {
//     fn from(error: &String) -> Self {
//         ErrorResponse {
//             errors: vec![error.into()],
//         }
//     }
// }
//
// impl From<Vec<String>> for ErrorResponse {
//     fn from(errors: Vec<String>) -> Self {
//         ErrorResponse { errors }
//     }
// }