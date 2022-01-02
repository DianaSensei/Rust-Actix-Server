use crate::utils::hasher::default_hasher_scheme_version;
use once_cell::sync::Lazy;

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Config {
    //pub auth_salt: String,
    pub cargo_pkg_name: String,
    pub jaeger_url: String,
    pub database_url: String,
    pub redis_url: String,
    pub nats_url: String,
    pub kafka_broker_url: String,
    pub kafka_message_timeout: String,
    //pub jwt_expiration: i64,
    //pub jwt_key: String,
    //pub rust_backtrace:u8,
    //pub rust_log:String,
    pub secret_key: String,
    pub server: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_host: String,
    pub smtp_port: i64,
    pub domain: String,
    pub dev_mode: bool,

    #[serde(default = "default_hasher_scheme_version")]
    pub scheme_hasher_version: usize,
    //pub session_key: String,
    //pub session_name:String,
    //pub session_secure: bool,
    //pub session_timeout: i64
}

pub static CONFIG: Lazy<Config> = Lazy::new(get_config);

fn get_config() -> Config {
    dotenv::dotenv().ok();

    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error),
    }
}
