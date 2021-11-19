use actix_web::client::Client;

pub fn get_rest_client() -> Client {
    Client::default()
}