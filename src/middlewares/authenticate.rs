// use crate::config::CONFIG;
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
// use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

pub struct Authentication;

impl FromRequest for Authentication {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let auth = req.headers().get("Authorization");
        match auth {
            Some(_) => {
                // let _split: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();
                // let _token = _split[1].trim();
                // let _var = &CONFIG.secret_key;
                // let key = _var.as_bytes();
                ok(Authentication)
            }
            None => err(ErrorUnauthorized("Authorization key is required")),
        }
    }
}

fn is_authorized(req: &HttpRequest) -> bool {
    if let Some(value) = req.headers().get("authorized") {
        // actual implementation that checks header here
        dbg!(value);
        true
    } else {
        false
    }
}
